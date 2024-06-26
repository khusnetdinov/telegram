use crate::bots_api::BotsApi;
use crate::structs::update::Update;
use crate::traits::storage::Storage;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

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
        Callback: Fn(BotsApi, Arc<STO>, Update) -> Fut + std::marker::Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait;
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
