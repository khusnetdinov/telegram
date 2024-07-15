use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_poll_option::InputPollOption as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputPollOption {
    pub text: Option<String>,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
impl From<Remote> for InputPollOption {
    fn from(remote: Remote) -> Self {
        Self {
            text: remote.text,
            text_parse_mode: remote.text_parse_mode,
            // TODO: #[remote(option, map, into)]
            text_entities: remote
                .text_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
