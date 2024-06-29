use crate::structs::options::send_options::SendOptions;

#[async_trait::async_trait]
pub trait Sender {
    // fn send_animation(&self) -> Message;
    // fn send_audio(&self) -> Message;
    // fn send_chat_action(&self) -> bool;
    // fn send_contact(&self) -> Message;

    async fn send_dice(
        &self,
        chat_id: i64,
        options: Option<SendOptions>,
    ) -> Result<(), Box<dyn std::error::Error>>;

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
