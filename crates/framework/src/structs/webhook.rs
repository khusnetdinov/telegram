use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Inner;
use telegram_bots_api::config::Config as InnerConfig;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Webhook {
    pub inner: Inner,
}

impl From<&InnerConfig> for Webhook {
    fn from(config: &InnerConfig) -> Self {
        Self {
            inner: Inner {
                url: config.webhook.to_string(),
                ..Default::default()
            },
        }
    }
}
