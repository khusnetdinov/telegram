use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl From<Remote> for WebhookInfo {
    fn from(remote: Remote) -> Self {
        Self {
            url: remote.url,
            has_custom_certificate: remote.has_custom_certificate,
            pending_update_count: remote.pending_update_count,
            ip_address: remote.ip_address,
            last_error_date: remote.last_error_date,
            last_error_message: remote.last_error_message,
            last_synchronization_error_date: remote.last_synchronization_error_date,
            max_connections: remote.max_connections,
            allowed_updates: remote.allowed_updates,
        }
    }
}
