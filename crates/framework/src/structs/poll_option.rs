use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::poll_option::PollOption as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}
impl From<Remote> for PollOption {
    fn from(remote: Remote) -> Self {
        Self {
            text: remote.text,
            voter_count: remote.voter_count,
            // TODO: #[remote(option, into)]
            text_entities: remote
                .text_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
