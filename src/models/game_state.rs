use std::collections::HashMap;

use super::card::Card;
use super::gem::{Gem, Token};
use super::noble::Noble;

/// Complete game state from a single player's perspective
#[derive(Debug, Clone)]
pub struct GameState {
    // Board state (shared)
    pub board_cards: Vec<Card>,        // visible cards on the board
    pub nobles: Vec<Noble>,            // available nobles
    pub tokens_available: HashMap<Token, u8>, // tokens in the bank

    // Player's reserved cards
    pub my_reserved: Vec<Card>,

    // Player's resources
    pub my_tokens: HashMap<Token, u8>,   // tokens the player holds
    pub my_bonuses: HashMap<Gem, u8>,    // bonuses from purchased cards

    // Player's purchased cards and score
    pub my_purchased_cards: Vec<Card>,
    pub my_points: u8,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board_cards: Vec::new(),
            nobles: Vec::new(),
            tokens_available: HashMap::new(),
            my_reserved: Vec::new(),
            my_tokens: HashMap::new(),
            my_bonuses: HashMap::new(),
            my_purchased_cards: Vec::new(),
            my_points: 0,
        }
    }

    /// Get total tokens the player is holding
    pub fn total_tokens(&self) -> u8 {
        self.my_tokens.values().sum()
    }

    /// Get player's effective purchasing power for a gem type
    /// (tokens + bonuses)
    pub fn effective_gems(&self, gem: Gem) -> u8 {
        let token_count = self.my_tokens.get(&gem.to_token()).copied().unwrap_or(0);
        let bonus_count = self.my_bonuses.get(&gem).copied().unwrap_or(0);
        token_count + bonus_count
    }

    /// Get number of gold tokens player has
    pub fn gold_tokens(&self) -> u8 {
        self.my_tokens.get(&Token::Gold).copied().unwrap_or(0)
    }

    /// Check if player has won (15+ points)
    pub fn has_won(&self) -> bool {
        self.my_points >= 15
    }

    /// Add a purchased card to player's collection
    pub fn purchase_card(&mut self, card: Card) {
        self.my_points += card.points;
        *self.my_bonuses.entry(card.bonus).or_insert(0) += 1;
        self.my_purchased_cards.push(card);
    }

    /// Calculate the total number of bonuses
    pub fn total_bonuses(&self) -> u8 {
        self.my_bonuses.values().sum()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
