use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Emoji {
    Dice,
    Darts,
    Bowling,
    Basketball,
    Football,
    SlotMachine,
}

impl Into<String> for Emoji {
    fn into(self) -> String {
        match self {
            Self::Dice => "🎲",
            Self::Darts => "🎯",
            Self::Bowling => "🎳",
            Self::Basketball => "🏀",
            Self::Football => "⚽",
            Self::SlotMachine => "🎰",
        }
        .to_string()
    }
}
