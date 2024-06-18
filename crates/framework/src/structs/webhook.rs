use crate::config::Config;
use crate::traits::params::Params;
use telegram_bots_api::api::params::delete_webhook::DeleteWebhook;
use telegram_bots_api::api::params::set_webhook::SetWebhook;

#[derive(Debug)]
pub struct Webhook {
    pub url: String,
    pub max_connections: Option<u32>,
    pub certificate: Option<String>,
    pub secret_token: Option<String>,
    pub ip_address: Option<String>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
}

impl From<&Config> for Webhook {
    fn from(config: &Config) -> Self {
        Self {
            url: config.webhook.clone(),
            max_connections: config.max_connections,
            certificate: config.certificate.clone(),
            secret_token: config.secret_token.clone(),
            ip_address: config.ip_address.clone(),
            allowed_updates: config.updates_allowed.clone(),
            drop_pending_updates: Some(config.updates_drop_pending),
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
            max_connections: self.max_connections,
            certificate: None,
            secret_token: self.secret_token.clone(),
            ip_address: self.ip_address.clone(),
            allowed_updates: self.allowed_updates.clone(),
            drop_pending_updates: self.drop_pending_updates,
        }
    }
}
