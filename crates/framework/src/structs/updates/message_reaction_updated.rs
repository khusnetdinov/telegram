use crate::enums::reaction_type::ReactionType;
use crate::structs::chat::Chat;
use crate::structs::message_id::MessageId;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_reaction_updated::MessageReactionUpdated as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
}
impl From<Remote> for MessageReactionUpdated {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            message_id: remote.message_id.into(),
            date: remote.date,
            // TODO: #[remote(map, into)]
            old_reaction: remote
                .old_reaction
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            // TODO: #[remote(map, into)]
            new_reaction: remote
                .new_reaction
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            // TODO: #[remote(option, into)]
            user: remote.user.map(|inner| inner.into()),
            // TODO: #[remote(option, into)]
            actor_chat: remote.actor_chat.map(|inner| inner.into()),
        }
    }
}
