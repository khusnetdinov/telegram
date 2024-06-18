use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Serialize, Deserialize, DerefInner, FromInner)]
pub struct WebhookInfo {
    pub inner: Inner,
}
