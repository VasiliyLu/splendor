use std::collections::HashMap;
use std::io::{self, Write};

use crate::data::{load_default_cards, load_default_nobles};
use crate::game::{suggest_all_moves, suggest_move};
use crate::models::{Card, GameState, Gem, Noble, Token};

pub fn run() -> io::Result<()> {
    println!("=== Splendor AI Advisor ===\n");

    // Load card and noble data
    let all_cards = load_default_cards()?;
    let all_nobles = load_default_nobles()?;

    println!("Loaded {} cards and {} nobles.\n", all_cards.len(), all_nobles.len());

    loop {
        println!("\n--- Main Menu ---");
        println!("1. Setup new game state");
        println!("2. Quick test (preset state)");
        println!("3. List all cards");
        println!("4. List all nobles");
        println!("5. Exit");
        print!("\nChoice: ");
        io::stdout().flush()?;

        let choice = read_line()?;
        match choice.trim() {
            "1" => setup_game_state(&all_cards, &all_nobles)?,
            "2" => quick_test(&all_cards, &all_nobles)?,
            "3" => list_cards(&all_cards),
            "4" => list_nobles(&all_nobles),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }

    Ok(())
}

fn setup_game_state(all_cards: &[Card], all_nobles: &[Noble]) -> io::Result<()> {
    let mut state = GameState::new();

    // 1. Enter tokens
    println!("\n--- Your Tokens ---");
    println!("Enter tokens as: onyx=2,sapphire=3,ruby=1 (or press Enter to skip)");
    print!("Tokens: ");
    io::stdout().flush()?;
    let tokens_input = read_line()?;
    state.my_tokens = parse_tokens(&tokens_input);
    println!("Tokens set: {:?}", state.my_tokens);

    // 2. Enter bonuses
    println!("\n--- Your Bonuses (from purchased cards) ---");
    println!("Enter bonuses as: ruby=1,diamond=2 (or press Enter to skip)");
    print!("Bonuses: ");
    io::stdout().flush()?;
    let bonuses_input = read_line()?;
    state.my_bonuses = parse_gems(&bonuses_input);
    println!("Bonuses set: {:?}", state.my_bonuses);

    // 3. Enter current points
    println!("\n--- Your Current Points ---");
    print!("Points (default 0): ");
    io::stdout().flush()?;
    let points_input = read_line()?;
    state.my_points = points_input.trim().parse().unwrap_or(0);
    println!("Points set: {}", state.my_points);

    // 4. Select cards on board
    println!("\n--- Board Cards ---");
    println!("Enter card IDs visible on the board, separated by commas (e.g., 1,5,12,45,71)");
    println!("(You can list cards with option 3 from main menu)");
    print!("Card IDs: ");
    io::stdout().flush()?;
    let cards_input = read_line()?;
    let card_ids: Vec<u32> = cards_input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    state.board_cards = all_cards
        .iter()
        .filter(|c| card_ids.contains(&c.id))
        .cloned()
        .collect();
    println!("Added {} cards to board", state.board_cards.len());

    // 5. Select nobles
    println!("\n--- Nobles on Board ---");
    println!("Enter noble IDs (1-10), separated by commas");
    print!("Noble IDs: ");
    io::stdout().flush()?;
    let nobles_input = read_line()?;
    let noble_ids: Vec<u32> = nobles_input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    state.nobles = all_nobles
        .iter()
        .filter(|n| noble_ids.contains(&n.id))
        .cloned()
        .collect();
    println!("Added {} nobles to board", state.nobles.len());

    // 6. Set available tokens (simplified - assume standard setup)
    println!("\n--- Available Tokens in Bank ---");
    println!("Enter available tokens (default: 7 of each, 5 gold)");
    println!("Format: onyx=5,sapphire=6,... or press Enter for default");
    print!("Bank tokens: ");
    io::stdout().flush()?;
    let bank_input = read_line()?;
    if bank_input.trim().is_empty() {
        for token in Token::NON_GOLD {
            state.tokens_available.insert(token, 7);
        }
        state.tokens_available.insert(Token::Gold, 5);
    } else {
        state.tokens_available = parse_tokens(&bank_input);
    }

    // Show state summary and get recommendation
    show_state_and_recommend(&state);

    Ok(())
}

fn quick_test(all_cards: &[Card], all_nobles: &[Noble]) -> io::Result<()> {
    println!("\n--- Quick Test ---");

    let mut state = GameState::new();

    // Set some tokens
    state.my_tokens.insert(Token::Ruby, 3);
    state.my_tokens.insert(Token::Sapphire, 2);
    state.my_tokens.insert(Token::Emerald, 1);

    // Set some bonuses
    state.my_bonuses.insert(Gem::Onyx, 1);
    state.my_bonuses.insert(Gem::Diamond, 1);

    // Add some board cards (first 12 - 4 of each level)
    state.board_cards = all_cards
        .iter()
        .filter(|c| c.level == 1)
        .take(4)
        .chain(all_cards.iter().filter(|c| c.level == 2).take(4))
        .chain(all_cards.iter().filter(|c| c.level == 3).take(4))
        .cloned()
        .collect();

    // Add some nobles
    state.nobles = all_nobles.iter().take(3).cloned().collect();

    // Set bank tokens
    for token in Token::NON_GOLD {
        state.tokens_available.insert(token, 5);
    }
    state.tokens_available.insert(Token::Gold, 5);

    show_state_and_recommend(&state);

    Ok(())
}

fn show_state_and_recommend(state: &GameState) {
    println!("\n========== GAME STATE ==========");
    println!("\nYour tokens: {:?}", state.my_tokens);
    println!("Your bonuses: {:?}", state.my_bonuses);
    println!("Your points: {}", state.my_points);
    println!("Reserved cards: {}", state.my_reserved.len());

    println!("\n--- Board Cards ---");
    for card in &state.board_cards {
        println!(
            "  #{}: Level {} | {} bonus | {} points | Cost: {:?}",
            card.id, card.level, format!("{:?}", card.bonus), card.points, card.costs
        );
    }

    println!("\n--- Nobles ---");
    for noble in &state.nobles {
        println!(
            "  #{}: {} points | Requirements: {:?}",
            noble.id, noble.points, noble.requirements
        );
    }

    println!("\n========== AI RECOMMENDATION ==========\n");

    let best_move = suggest_move(state);
    println!(">>> BEST MOVE: {}", best_move.mv);
    println!("    Score: {}", best_move.score);
    println!("    Reasons:");
    for reason in &best_move.reasons {
        println!("      - {}", reason);
    }

    println!("\n--- Top 5 Moves ---");
    let all_moves = suggest_all_moves(state);
    for (i, scored) in all_moves.iter().take(5).enumerate() {
        println!("{}. {} (score: {})", i + 1, scored.mv, scored.score);
    }
}

fn list_cards(cards: &[Card]) {
    println!("\n=== ALL CARDS ===\n");

    for level in 1..=3 {
        println!("--- Level {} ---", level);
        for card in cards.iter().filter(|c| c.level == level) {
            println!(
                "  #{:2}: {:?} bonus | {} pts | {:?}",
                card.id, card.bonus, card.points, card.costs
            );
        }
        println!();
    }
}

fn list_nobles(nobles: &[Noble]) {
    println!("\n=== ALL NOBLES ===\n");
    for noble in nobles {
        println!(
            "  #{:2}: {} points | Requirements: {:?}",
            noble.id, noble.points, noble.requirements
        );
    }
}

fn parse_tokens(input: &str) -> HashMap<Token, u8> {
    let mut tokens = HashMap::new();
    for part in input.split(',') {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            let token = match name.trim().to_lowercase().as_str() {
                "onyx" | "black" => Some(Token::Onyx),
                "sapphire" | "blue" => Some(Token::Sapphire),
                "emerald" | "green" => Some(Token::Emerald),
                "ruby" | "red" => Some(Token::Ruby),
                "diamond" | "white" => Some(Token::Diamond),
                "gold" | "yellow" => Some(Token::Gold),
                _ => None,
            };
            if let (Some(t), Ok(v)) = (token, value.trim().parse::<u8>()) {
                tokens.insert(t, v);
            }
        }
    }
    tokens
}

fn parse_gems(input: &str) -> HashMap<Gem, u8> {
    let mut gems = HashMap::new();
    for part in input.split(',') {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            let gem = match name.trim().to_lowercase().as_str() {
                "onyx" | "black" => Some(Gem::Onyx),
                "sapphire" | "blue" => Some(Gem::Sapphire),
                "emerald" | "green" => Some(Gem::Emerald),
                "ruby" | "red" => Some(Gem::Ruby),
                "diamond" | "white" => Some(Gem::Diamond),
                _ => None,
            };
            if let (Some(g), Ok(v)) = (gem, value.trim().parse::<u8>()) {
                gems.insert(g, v);
            }
        }
    }
    gems
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
