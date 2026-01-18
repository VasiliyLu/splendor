use std::collections::HashMap;
use std::fmt;

use crate::models::Token;

/// Represents a possible move in Splendor
#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    /// Take up to 3 different tokens (one of each color)
    TakeTokens(HashMap<Token, u8>),

    /// Take 2 tokens of the same color (if 4+ available)
    TakeTwoTokens(Token),

    /// Buy a card from the board or reserved hand
    BuyCard {
        card_id: u32,
        gold_used: u8,
        from_reserved: bool,
    },

    /// Reserve a card and take a gold token
    ReserveCard { card_id: u32 },

    /// Pass (only when no other moves available)
    Pass,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::TakeTokens(tokens) => {
                let tokens_str: Vec<String> = tokens
                    .iter()
                    .filter(|&(_, count)| *count > 0)
                    .map(|(t, _)| format!("{:?}", t))
                    .collect();
                write!(f, "Take tokens: {}", tokens_str.join(", "))
            }
            Move::TakeTwoTokens(token) => {
                write!(f, "Take 2 {:?} tokens", token)
            }
            Move::BuyCard {
                card_id,
                gold_used,
                from_reserved,
            } => {
                let source = if *from_reserved { "reserved" } else { "board" };
                if *gold_used > 0 {
                    write!(
                        f,
                        "Buy card #{} from {} (using {} gold)",
                        card_id, source, gold_used
                    )
                } else {
                    write!(f, "Buy card #{} from {}", card_id, source)
                }
            }
            Move::ReserveCard { card_id } => {
                write!(f, "Reserve card #{}", card_id)
            }
            Move::Pass => write!(f, "Pass"),
        }
    }
}
