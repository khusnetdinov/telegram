use crate::structs::reactions::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::structs::reactions::reaction_type_emoji::ReactionTypeEmoji;
use crate::structs::reactions::reaction_type_paid::ReactionTypePaid;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::reaction_type::ReactionType as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReactionType {
    CustomEmoji(ReactionTypeCustomEmoji),
    Emoji(ReactionTypeEmoji),
    Paid(ReactionTypePaid),
}

impl From<Remote> for ReactionType {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::CustomEmoji(reaction) => Self::CustomEmoji(reaction.into()),
            Remote::Emoji(reaction) => Self::Emoji(reaction.into()),
            Remote::Paid(reaction) => Self::Paid(reaction.into()),
        }
    }
}

impl Default for ReactionType {
    fn default() -> Self {
        Self::Emoji(ReactionTypeEmoji {
            ..Default::default()
        })
    }
}
