use std::collections::HashMap;

use crate::models::{Card, GameState, Token};

use super::moves::Move;

/// Result of checking if a card can be afforded
pub struct AffordResult {
    pub can_afford: bool,
    pub gold_needed: u8,
}

impl GameState {
    /// Check if the player can afford a card and how much gold is needed
    pub fn can_afford(&self, card: &Card) -> AffordResult {
        let mut gold_needed: u8 = 0;

        for (gem, &cost) in &card.costs {
            let bonus = self.my_bonuses.get(gem).copied().unwrap_or(0);
            let tokens = self.my_tokens.get(&gem.to_token()).copied().unwrap_or(0);
            let total_available = bonus + tokens;

            if cost > total_available {
                gold_needed += cost - total_available;
            }
        }

        let gold_available = self.gold_tokens();
        AffordResult {
            can_afford: gold_needed <= gold_available,
            gold_needed: gold_needed.min(gold_available),
        }
    }

    /// Generate all legal moves for the current game state
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        // 1. Buy card moves (from board)
        for card in &self.board_cards {
            let afford = self.can_afford(card);
            if afford.can_afford {
                moves.push(Move::BuyCard {
                    card_id: card.id,
                    gold_used: afford.gold_needed,
                    from_reserved: false,
                });
            }
        }

        // 2. Buy card moves (from reserved)
        for card in &self.my_reserved {
            let afford = self.can_afford(card);
            if afford.can_afford {
                moves.push(Move::BuyCard {
                    card_id: card.id,
                    gold_used: afford.gold_needed,
                    from_reserved: true,
                });
            }
        }

        // 3. Take tokens moves (up to 3 different)
        if self.total_tokens() < 10 {
            let available_non_gold: Vec<Token> = Token::NON_GOLD
                .iter()
                .filter(|t| self.tokens_available.get(t).copied().unwrap_or(0) > 0)
                .copied()
                .collect();

            // Take 3 different tokens
            if available_non_gold.len() >= 3 {
                self.generate_take_three_combinations(&available_non_gold, &mut moves);
            }

            // Take 2 different tokens
            if available_non_gold.len() >= 2 {
                self.generate_take_two_combinations(&available_non_gold, &mut moves);
            }

            // Take 1 token
            for token in &available_non_gold {
                let mut tokens = HashMap::new();
                tokens.insert(*token, 1);
                moves.push(Move::TakeTokens(tokens));
            }

            // 4. Take 2 of the same token (if 4+ available)
            for token in Token::NON_GOLD {
                if self.tokens_available.get(&token).copied().unwrap_or(0) >= 4 {
                    moves.push(Move::TakeTwoTokens(token));
                }
            }
        }

        // 5. Reserve card moves (if < 3 reserved)
        if self.my_reserved.len() < 3 {
            for card in &self.board_cards {
                moves.push(Move::ReserveCard { card_id: card.id });
            }
        }

        // 6. If no moves available, add Pass
        if moves.is_empty() {
            moves.push(Move::Pass);
        }

        moves
    }

    fn generate_take_three_combinations(&self, available: &[Token], moves: &mut Vec<Move>) {
        let n = available.len();
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let mut tokens = HashMap::new();
                    tokens.insert(available[i], 1);
                    tokens.insert(available[j], 1);
                    tokens.insert(available[k], 1);
                    moves.push(Move::TakeTokens(tokens));
                }
            }
        }
    }

    fn generate_take_two_combinations(&self, available: &[Token], moves: &mut Vec<Move>) {
        let n = available.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let mut tokens = HashMap::new();
                tokens.insert(available[i], 1);
                tokens.insert(available[j], 1);
                moves.push(Move::TakeTokens(tokens));
            }
        }
    }

    /// Get all affordable cards
    pub fn affordable_cards(&self) -> Vec<&Card> {
        self.board_cards
            .iter()
            .chain(self.my_reserved.iter())
            .filter(|card| self.can_afford(card).can_afford)
            .collect()
    }

    /// Check if player can claim a noble
    pub fn claimable_nobles(&self) -> Vec<usize> {
        self.nobles
            .iter()
            .enumerate()
            .filter_map(|(i, noble)| {
                let can_claim = noble.requirements.iter().all(|(gem, &required)| {
                    self.my_bonuses.get(gem).copied().unwrap_or(0) >= required
                });
                if can_claim {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
}
