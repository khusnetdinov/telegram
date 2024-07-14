use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::user::User as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub an_connect_to_business: Option<bool>,
}

impl From<Remote> for User {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            is_bot: remote.is_bot,
            first_name: remote.first_name,
            last_name: remote.last_name,
            username: remote.username,
            language_code: remote.language_code,
            is_premium: remote.is_premium,
            added_to_attachment_menu: remote.added_to_attachment_menu,
            can_join_groups: remote.can_join_groups,
            can_read_all_group_messages: remote.can_read_all_group_messages,
            supports_inline_queries: remote.supports_inline_queries,
            an_connect_to_business: remote.an_connect_to_business,
        }
    }
}
