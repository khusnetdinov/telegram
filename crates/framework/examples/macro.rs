use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    pub ip_address: Option<String>,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<i64>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
}
