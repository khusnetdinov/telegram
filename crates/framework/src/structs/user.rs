use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::user::User as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
}

impl From<User> for Remote {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            is_bot: value.is_bot,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            language_code: value.language_code,
            is_premium: value.is_premium,
            added_to_attachment_menu: value.added_to_attachment_menu,
            can_join_groups: value.can_join_groups,
            can_read_all_group_messages: value.can_read_all_group_messages,
            supports_inline_queries: value.supports_inline_queries,
            an_connect_to_business: value.an_connect_to_business,
            has_main_web_app: value.has_main_web_app,
        }
    }
}
