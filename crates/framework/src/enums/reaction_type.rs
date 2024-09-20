use crate::structs::message_reactions::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::structs::message_reactions::reaction_type_emoji::ReactionTypeEmoji;
use crate::structs::message_reactions::reaction_type_paid::ReactionTypePaid;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::reaction_type::ReactionType as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
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
