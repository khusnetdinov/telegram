use telegram_bots_api::api::structs::business_connection::BusinessConnection as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub can_reply: bool,
    pub is_enabled: bool,
}
