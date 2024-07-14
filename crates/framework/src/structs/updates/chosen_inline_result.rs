use crate::structs::messages::location::Location;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chosen_inline_result::ChosenInlineResult as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
impl From<Remote> for ChosenInlineResult {
    fn from(remote: Remote) -> Self {
        Self {
            result_id: remote.result_id,
            // TODO: #[remote(into)]
            from: remote.from.into(),
            query: remote.query,
            // TODO: #[remote(option, into)]
            location: remote.location.map(|inner| inner.into()),
            inline_message_id: remote.inline_message_id,
        }
    }
}
