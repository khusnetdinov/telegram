use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_emoji::ReactionTypeEmoji as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub emoji: String,
}
impl From<Remote> for ReactionTypeEmoji {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            emoji: remote.emoji,
        }
    }
}
