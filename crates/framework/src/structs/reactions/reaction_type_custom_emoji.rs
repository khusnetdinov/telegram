use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_custom_emoji::ReactionTypeCustomEmoji as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeCustomEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub custom_emoji_id: String,
}
impl From<Remote> for ReactionTypeCustomEmoji {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            custom_emoji_id: remote.custom_emoji_id,
        }
    }
}
