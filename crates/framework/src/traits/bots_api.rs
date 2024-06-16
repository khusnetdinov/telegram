use crate::bots_api::BotsApi;
use crate::structs::bot_command::BotCommand;
use crate::structs::update::Update;
use crate::structs::webhook::Webhook;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;

pub trait Commander {
    fn commands(&self, params: (DeleteMyCommands, GetMyCommands, SetMyCommands));
    fn delete_commands(&self, params: DeleteMyCommands) -> bool;
    fn get_commands(&self, params: GetMyCommands) -> Vec<BotCommand>;
    fn set_commands(&self, params: SetMyCommands) -> bool;
}

pub trait HttpListener {
    fn http_listen(&self);
}

pub trait HttpsListener {
    fn https_listen(&self);
}

pub trait Pooler {
    fn pooling(&self, callback: impl Fn(&BotsApi, Update));
}

pub trait Webhooker {
    fn delete_webhook(&self) -> bool;
    fn get_webhook(&self) -> Webhook;
    fn set_webhook(&self) -> bool;
}
