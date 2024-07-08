use telegram_bots_api::api::structs::chat::Chat as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct Chat {
    pub kind: String,
    pub id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
}
