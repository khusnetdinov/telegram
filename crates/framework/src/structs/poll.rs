use crate::structs::message_entity::MessageEntity;
use crate::structs::polls::poll_option::PollOption;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::poll::Poll as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl From<Remote> for Poll {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            id: remote.id,
            question: remote.question,
            // TODO: #[remote(map)]
            options: remote
                .options
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            total_voter_count: remote.total_voter_count,
            is_closed: remote.is_closed,
            is_anonymous: remote.is_anonymous,
            allows_multiple_answers: remote.allows_multiple_answers,
            // TODO: #[remote(option, map, into)]
            question_entities: remote
                .question_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            correct_option_id: remote.correct_option_id,
            explanation: remote.explanation,
            // TODO: #[remote(option, map, into)]
            explanation_entities: remote
                .explanation_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            open_period: remote.open_period,
            close_date: remote.close_date,
        }
    }
}

impl From<Message> for Poll {
    fn from(remote: Message) -> Self {
        let Message {
            poll: Some(poll), ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(poll)
    }
}
