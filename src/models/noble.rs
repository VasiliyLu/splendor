use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::gem::Gem;

/// A Noble tile that can be claimed when requirements are met
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Noble {
    pub id: u32,
    pub points: u8, // always 3 in standard game
    pub requirements: HashMap<Gem, u8>,
}

impl Noble {
    pub fn new(id: u32, requirements: HashMap<Gem, u8>) -> Self {
        Self {
            id,
            points: 3,
            requirements,
        }
    }
}
