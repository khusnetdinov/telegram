use crate::structs::bot_command::BotCommand;
use crate::structs::update::Update;
use crate::structs::webhook_info::WebhookInfo;
use crate::traits::storage::Storage;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;

#[async_trait::async_trait]
pub trait Commander {
    async fn commands(
        &self,
        params: (DeleteMyCommands, GetMyCommands, SetMyCommands),
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn delete_commands(
        &self,
        params: DeleteMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_commands(
        &self,
        params: GetMyCommands,
    ) -> Result<Vec<BotCommand>, Box<dyn std::error::Error>>;

    async fn set_commands(&self, params: SetMyCommands)
        -> Result<bool, Box<dyn std::error::Error>>;
}

#[async_trait::async_trait]
pub trait HttpListener {
    fn http_listen(&self);
}

#[async_trait::async_trait]
pub trait HttpsListener {
    fn https_listen(&self);
}

#[async_trait::async_trait]
pub trait Pooler<STO, STA> {
    async fn pooling<Callback, Fut>(
        &self,
        storage: Arc<STO>,
        callback: Callback,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Callback: Fn(Arc<STO>, Update) -> Fut + std::marker::Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait;
}

#[async_trait::async_trait]
pub trait Webhooker {
    async fn delete_webhook(&self) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_webhook(&self) -> Result<WebhookInfo, Box<dyn std::error::Error>>;

    async fn set_webhook(&self) -> Result<bool, Box<dyn std::error::Error>>;
}

#[async_trait::async_trait]
pub trait Sender {
    // fn send_animation(&self) -> Message;
    // fn send_audio(&self) -> Message;
    // fn send_chat_action(&self) -> bool;
    // fn send_contact(&self) -> Message;
    // fn send_dice(&self, chat_id: i64);
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
