use crate::structs::reactions::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::structs::reactions::reaction_type_emoji::ReactionTypeEmoji;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::reaction_type::ReactionType as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
}

impl From<Remote> for ReactionType {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Emoji(reaction) => Self::Emoji(reaction.into()),
            Remote::CustomEmoji(reaction) => Self::CustomEmoji(reaction.into()),
        }
    }
}
