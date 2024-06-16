use crate::config::Config;
use crate::traits::params::Params;
use telegram_bots_api::api::params::set_webhook::SetWebhook;
use telegram_bots_api::api::structs::webhook_info::WebhookInfo as Inner;
use telegram_bots_api::api::{
    params::delete_webhook::DeleteWebhook, structs::input_file::InputFile,
};
use telegram_macros::DerefInner;

#[derive(Debug, DerefInner)]
pub struct Webhook {
    pub inner: Inner,
    // TODO: Config
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
                ..Default::default()
            },
            drop_pending_updates: None,
            certificate: None,
            secret_token: None,
        }
    }
}

impl Params for Webhook {
    type Delete = DeleteWebhook;
    type Get = ();
    type Set = SetWebhook;

    fn delete_params(&self) -> Self::Delete {
        Self::Delete {
            drop_pending_updates: Some(true),
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
            drop_pending_updates: self.drop_pending_updates,
            secret_token: self.secret_token.clone(),
        }
    }
}
