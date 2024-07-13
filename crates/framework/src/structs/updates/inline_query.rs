use crate::structs::messages::location::Location;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_query::InlineQuery as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
impl From<Remote> for InlineQuery {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            // TODO: #[remote(into)]
            from: remote.from.into(),
            query: remote.query,
            offset: remote.offset,
            chat_type: remote.chat_type,
            // TODO: #[remote(option)]
            location: remote.location.map(|inner| inner.into()),
        }
    }
}
