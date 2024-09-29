use crate::structs::chat::Chat;
use crate::structs::messages::message_id::MessageId;
use crate::structs::messages::message_reactions::reaction_count::ReactionCount;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_reaction_count_update::MessageReactionCountUpdated as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}
