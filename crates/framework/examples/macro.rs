use telegram_bots_api::api::structs::inline_query::InlineQuery as Remote;

use telegram_bots_api::api::structs::location::Location;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
