use crate::structs::reactions::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::structs::reactions::reaction_type_emoji::ReactionTypeEmoji;
use crate::structs::reactions::reaction_type_paid::ReactionTypePaid;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::reaction_type::ReactionType as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
pub enum ReactionType {
    CustomEmoji(ReactionTypeCustomEmoji),
    Emoji(ReactionTypeEmoji),
    Paid(ReactionTypePaid),
}

impl Default for ReactionType {
    fn default() -> Self {
        Self::Emoji(ReactionTypeEmoji {
            ..Default::default()
        })
    }
}
