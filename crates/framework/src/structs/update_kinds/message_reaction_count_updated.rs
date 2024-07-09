use crate::structs::chat::Chat;
use crate::structs::message_id::MessageId;
use crate::structs::reactions::reaction_count::ReactionCount;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_reaction_count_update::MessageReactionCountUpdated as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}
impl From<Remote> for MessageReactionCountUpdated {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            message_id: remote.message_id.into(),
            date: remote.date,
            // TODO: #[remote(map)]
            reactions: remote
                .reactions
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}
