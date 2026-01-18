use serde::{Deserialize, Serialize};

/// The five gem types that cards provide as bonuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gem {
    Onyx, //black
    Sapphire, //blue
    Emerald, //green
    Ruby, //red
    Diamond, //white
}

/// Token types including Gold (wildcard)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Token {
    Onyx,
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Gold,
}

impl Gem {
    /// Convert a Gem to its corresponding Token
    pub fn to_token(self) -> Token {
        match self {
            Gem::Onyx => Token::Onyx,
            Gem::Sapphire => Token::Sapphire,
            Gem::Emerald => Token::Emerald,
            Gem::Ruby => Token::Ruby,
            Gem::Diamond => Token::Diamond,
        }
    }

    /// All gem types (useful for iteration)
    pub const ALL: [Gem; 5] = [Gem::Onyx, Gem::Sapphire, Gem::Emerald, Gem::Ruby, Gem::Diamond];
}

impl Token {
    /// Try to convert a Token to a Gem (fails for Gold)
    pub fn to_gem(self) -> Option<Gem> {
        match self {
            Token::Onyx => Some(Gem::Onyx),
            Token::Sapphire => Some(Gem::Sapphire),
            Token::Emerald => Some(Gem::Emerald),
            Token::Ruby => Some(Gem::Ruby),
            Token::Diamond => Some(Gem::Diamond),
            Token::Gold => None,
        }
    }

    /// Check if this token is Gold (wildcard)
    pub fn is_gold(self) -> bool {
        matches!(self, Token::Gold)
    }

    /// All token types (useful for iteration)
    pub const ALL: [Token; 6] = [
        Token::Onyx,
        Token::Sapphire,
        Token::Emerald,
        Token::Ruby,
        Token::Diamond,
        Token::Gold,
    ];

    /// All non-gold token types
    pub const NON_GOLD: [Token; 5] = [
        Token::Onyx,
        Token::Sapphire,
        Token::Emerald,
        Token::Ruby,
        Token::Diamond,
    ];
}

impl From<Gem> for Token {
    fn from(gem: Gem) -> Self {
        gem.to_token()
    }
}

impl TryFrom<Token> for Gem {
    type Error = ();

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        token.to_gem().ok_or(())
    }
}
