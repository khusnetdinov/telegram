use crate::bots_api::BotsApi;
use crate::structs::bot_command::BotCommand;
use crate::structs::update::Update;
use crate::structs::webhook_info::WebhookInfo;
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
    fn get_webhook(&self) -> WebhookInfo;
    fn set_webhook(&self) -> bool;
}

pub trait Sender {
    // fn send_animation(&self) -> Message;
    // fn send_audio(&self) -> Message;
    // fn send_chat_action(&self) -> bool;
    // fn send_contact(&self) -> Message;
    fn send_dice(&self, chat_id: i64);
    // fn send_document(&self) -> Message;
    // fn send_game(&self) -> Message;
    // fn send_invoice(&self) -> Message;
    // fn send_location(&self) -> Mesasge;
    // fn send_media_group(&self) -> Vec<Message>;
    // fn send_message(&self) -> Message;
    // fn send_photo(&self) -> Message;
    // fn send_poll(&self) -> Message;
    // fn send_venue(&self) -> Message;
    // fn send_video(&self) -> Message;
    // fn send_video_note(&self) -> Message;
    // fn send_voice(&self) -> Message;
}
