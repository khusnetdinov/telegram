use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_poll_option::InputPollOption as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InputPollOption {
    pub text: Option<String>,
    pub text_parse_mode: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
}
