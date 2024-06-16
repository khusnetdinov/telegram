use crate::config::Config;
use crate::traits::params::Params;
use telegram_bots_api::api::params::delete_webhook::DeleteWebhook;
use telegram_bots_api::api::params::set_webhook::SetWebhook;
use telegram_bots_api::api::structs::input_file::InputFile;
use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Inner;
use telegram_macros::DerefInner;

#[derive(Debug, DerefInner)]
pub struct Webhook {
    pub inner: Inner,
    pub drop_pending_updates: Option<bool>,
    pub certificate: Option<InputFile>,
    pub secret_token: Option<String>,
}

impl From<Inner> for Webhook {
    fn from(inner: Inner) -> Self {
        Self {
            inner,
            drop_pending_updates: None,
            certificate: None,
            secret_token: None,
        }
    }
}

impl From<&Config> for Webhook {
    fn from(config: &Config) -> Self {
        Self {
            inner: Inner {
                url: config.webhook.to_string(),
                has_custom_certificate: config.certificate.is_some(),
                ..Default::default()
            },
            drop_pending_updates: config.drop_pending_updates,
            certificate: config.certificate.clone(),
            secret_token: config.secret_token.clone(),
        }
    }
}

impl Params for Webhook {
    type Delete = DeleteWebhook;
    type Get = ();
    type Set = SetWebhook;

    fn delete_params(&self) -> Self::Delete {
        Self::Delete {
            drop_pending_updates: self.drop_pending_updates,
        }
    }

    fn get_params(&self) -> Self::Get {
        unimplemented!()
    }

    fn set_params(&self) -> Self::Set {
        Self::Set {
            url: self.url.clone(),
            certificate: self.certificate.clone(),
            ip_address: self.ip_address.clone(),
            max_connections: self.max_connections,
            allowed_updates: self.allowed_updates.clone(),
            secret_token: self.secret_token.clone(),
            drop_pending_updates: self.drop_pending_updates,
        }
    }
}
