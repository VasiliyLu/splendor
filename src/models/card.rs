use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::gem::Gem;

/// A development card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub level: u8, // 1, 2, or 3
    pub points: u8,
    pub costs: HashMap<Gem, u8>,
    pub bonus: Gem,
}

impl Card {
    pub fn new(id: u32, level: u8, points: u8, costs: HashMap<Gem, u8>, bonus: Gem) -> Self {
        Self {
            id,
            level,
            points,
            costs,
            bonus,
        }
    }

    /// Calculate the total cost of this card
    pub fn total_cost(&self) -> u8 {
        self.costs.values().sum()
    }
}
