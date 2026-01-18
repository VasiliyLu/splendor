use std::collections::HashMap;

use crate::models::{Card, GameState, Token};

use super::moves::Move;

/// A move with its calculated score and reasoning
#[derive(Debug)]
pub struct ScoredMove {
    pub mv: Move,
    pub score: i32,
    pub reasons: Vec<String>,
}

impl ScoredMove {
    fn new(mv: Move) -> Self {
        Self {
            mv,
            score: 0,
            reasons: Vec::new(),
        }
    }

    fn add_score(&mut self, points: i32, reason: &str) {
        self.score += points;
        self.reasons.push(format!("{} (+{})", reason, points));
    }
}

/// Suggest the best move for the current game state
pub fn suggest_move(state: &GameState) -> ScoredMove {
    let moves = state.generate_moves();

    if moves.is_empty() {
        return ScoredMove::new(Move::Pass);
    }

    let mut scored_moves: Vec<ScoredMove> = moves
        .into_iter()
        .map(|mv| score_move(state, mv))
        .collect();

    scored_moves.sort_by(|a, b| b.score.cmp(&a.score));
    scored_moves.into_iter().next().unwrap()
}

fn score_move(state: &GameState, mv: Move) -> ScoredMove {
    let mut scored = ScoredMove::new(mv.clone());

    match &mv {
        Move::BuyCard {
            card_id,
            gold_used,
            from_reserved,
        } => {
            score_buy_card(state, *card_id, *gold_used, *from_reserved, &mut scored);
        }
        Move::TakeTokens(tokens) => {
            score_take_tokens(state, tokens, &mut scored);
        }
        Move::TakeTwoTokens(token) => {
            score_take_two_tokens(state, *token, &mut scored);
        }
        Move::ReserveCard { card_id } => {
            score_reserve_card(state, *card_id, &mut scored);
        }
        Move::Pass => {
            scored.add_score(-100, "Pass (no other options)");
        }
    }

    scored
}

fn score_buy_card(
    state: &GameState,
    card_id: u32,
    gold_used: u8,
    from_reserved: bool,
    scored: &mut ScoredMove,
) {
    let card = find_card(state, card_id, from_reserved);

    if let Some(card) = card {
        // 1. Winning move check
        let potential_points = state.my_points + card.points;
        if potential_points >= 15 {
            scored.add_score(100, "Winning move!");
        }

        // 2. Points from card
        if card.points > 0 {
            scored.add_score(card.points as i32 * 10, &format!("{} points", card.points));
        }

        // 3. Progress toward noble
        let noble_progress = calculate_noble_progress(state, card);
        if noble_progress > 0 {
            scored.add_score(noble_progress, "Progress toward noble");
        }

        // 4. Engine building (level 1 cards early, level 3 late)
        let total_bonuses = state.total_bonuses();
        if card.level == 1 && total_bonuses < 8 {
            scored.add_score(3, "Engine building (early level 1)");
        } else if card.level == 2 && total_bonuses >= 4 {
            scored.add_score(2, "Mid-game level 2");
        }

        // 5. Penalty for gold usage
        if gold_used > 0 {
            scored.add_score(-(gold_used as i32), &format!("Uses {} gold", gold_used));
        }

        // 6. Bonus for buying from reserved
        if from_reserved {
            scored.add_score(1, "Clears reserved slot");
        }

        // 7. Base score for buying any card
        scored.add_score(5, "Buy card action");
    }
}

fn score_take_tokens(state: &GameState, tokens: &HashMap<Token, u8>, scored: &mut ScoredMove) {
    let token_count: u8 = tokens.values().sum();

    // Base score for taking tokens
    scored.add_score(token_count as i32, &format!("Take {} tokens", token_count));

    // Check if tokens help afford cards on board
    let usefulness = calculate_token_usefulness(state, tokens);
    if usefulness > 0 {
        scored.add_score(usefulness, "Tokens useful for board cards");
    }
}

fn score_take_two_tokens(state: &GameState, token: Token, scored: &mut ScoredMove) {
    scored.add_score(2, "Take 2 same tokens");

    // Check usefulness
    let mut tokens = HashMap::new();
    tokens.insert(token, 2);
    let usefulness = calculate_token_usefulness(state, &tokens);
    if usefulness > 0 {
        scored.add_score(usefulness, "Tokens useful for board cards");
    }
}

fn score_reserve_card(state: &GameState, card_id: u32, scored: &mut ScoredMove) {
    let card = state.board_cards.iter().find(|c| c.id == card_id);

    if let Some(card) = card {
        // Base score for reserve
        scored.add_score(2, "Reserve action");

        // Gold token bonus (if available)
        let gold_available = state.tokens_available.get(&Token::Gold).copied().unwrap_or(0);
        if gold_available > 0 {
            scored.add_score(2, "Gains gold token");
        }

        // High-value card bonus
        if card.points >= 4 {
            scored.add_score(5, "High-value card (4+ points)");
        } else if card.points >= 3 {
            scored.add_score(3, "Good card (3 points)");
        }

        // Penalty if already have 2 reserved
        if state.my_reserved.len() >= 2 {
            scored.add_score(-3, "Already have reserved cards");
        }
    }
}

fn find_card(state: &GameState, card_id: u32, from_reserved: bool) -> Option<&Card> {
    if from_reserved {
        state.my_reserved.iter().find(|c| c.id == card_id)
    } else {
        state.board_cards.iter().find(|c| c.id == card_id)
    }
}

fn calculate_noble_progress(state: &GameState, card: &Card) -> i32 {
    let mut progress = 0;

    for noble in &state.nobles {
        // Check if the card's bonus helps toward this noble
        if let Some(&required) = noble.requirements.get(&card.bonus) {
            let current = state.my_bonuses.get(&card.bonus).copied().unwrap_or(0);
            if current < required {
                // This card helps toward this noble
                progress += 5;

                // Extra bonus if it completes the requirement
                if current + 1 >= required {
                    let would_complete = noble.requirements.iter().all(|(gem, &req)| {
                        let have = state.my_bonuses.get(gem).copied().unwrap_or(0);
                        let add = if *gem == card.bonus { 1 } else { 0 };
                        have + add >= req
                    });
                    if would_complete {
                        progress += 10;
                    }
                }
            }
        }
    }

    progress
}

fn calculate_token_usefulness(state: &GameState, tokens: &HashMap<Token, u8>) -> i32 {
    let mut usefulness = 0;

    for card in &state.board_cards {
        for (gem, &cost) in &card.costs {
            let bonus = state.my_bonuses.get(gem).copied().unwrap_or(0);
            let current_tokens = state.my_tokens.get(&gem.to_token()).copied().unwrap_or(0);
            let new_tokens = tokens.get(&gem.to_token()).copied().unwrap_or(0);

            if cost > bonus + current_tokens && new_tokens > 0 {
                // These tokens help toward this card
                usefulness += new_tokens as i32;
            }
        }
    }

    usefulness.min(6)
}

/// Get all suggestions sorted by score
pub fn suggest_all_moves(state: &GameState) -> Vec<ScoredMove> {
    let moves = state.generate_moves();

    let mut scored_moves: Vec<ScoredMove> = moves
        .into_iter()
        .map(|mv| score_move(state, mv))
        .collect();

    scored_moves.sort_by(|a, b| b.score.cmp(&a.score));
    scored_moves
}
