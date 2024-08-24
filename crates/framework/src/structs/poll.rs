use crate::structs::message_entity::MessageEntity;
use crate::structs::polls::poll_option::PollOption;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::poll::Poll as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct Poll {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

impl From<Message> for Poll {
    fn from(remote: Message) -> Self {
        let Message { poll, .. } = remote;

        Self::from(poll.unwrap())
    }
}
