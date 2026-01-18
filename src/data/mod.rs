use std::fs;
use std::io;
use std::path::Path;

use crate::models::{Card, Noble};

/// Load all cards from a JSON file
pub fn load_cards<P: AsRef<Path>>(path: P) -> io::Result<Vec<Card>> {
    let content = fs::read_to_string(path)?;
    let cards: Vec<Card> = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(cards)
}

/// Load all nobles from a JSON file
pub fn load_nobles<P: AsRef<Path>>(path: P) -> io::Result<Vec<Noble>> {
    let content = fs::read_to_string(path)?;
    let nobles: Vec<Noble> = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(nobles)
}

/// Load cards from the default embedded data path
pub fn load_default_cards() -> io::Result<Vec<Card>> {
    load_cards("data/cards.json")
}

/// Load nobles from the default embedded data path
pub fn load_default_nobles() -> io::Result<Vec<Noble>> {
    load_nobles("data/nobles.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cards() {
        let cards = load_default_cards().expect("Failed to load cards");
        assert_eq!(cards.len(), 90);

        // Check level distribution
        let level1 = cards.iter().filter(|c| c.level == 1).count();
        let level2 = cards.iter().filter(|c| c.level == 2).count();
        let level3 = cards.iter().filter(|c| c.level == 3).count();
        assert_eq!(level1, 40);
        assert_eq!(level2, 30);
        assert_eq!(level3, 20);
    }

    #[test]
    fn test_load_nobles() {
        let nobles = load_default_nobles().expect("Failed to load nobles");
        assert_eq!(nobles.len(), 10);

        // All nobles should have 3 points
        for noble in &nobles {
            assert_eq!(noble.points, 3);
        }
    }
}
