use crate::structs::config::Config;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::api::types::user::User;
use telegram_bots_api::clients::sync::Sync;

#[derive(Debug)]
pub struct BotsApi {
    client: Sync,
    user: User,
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let client = Sync::from(&*config);
        let user = client.get_me().unwrap();

        Self { client, user }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let client = Sync::new();
        let user = client.get_me().unwrap();

        Self { client, user }
    }

    pub fn pooling(&self) {
        todo!()
    }

    pub fn listen_http(&self) {
        todo!()
    }

    pub fn listen_https(&self) {
        todo!()
    }
}
