#[async_trait::async_trait]
pub trait Message {
    // copy_message
    // copy_messages
    // delete_message
    // delete_messages
    // edit_message_caption
    // edit_message_live_location
    // edit_message_media
    // edit_message_reply_markup
    // edit_message_text
    // forward_message
    // forward_messages
    // set_message_reaction
    // stop_message_live_location

    // async fn send_message(
    //     &self,
    //     chat_id: i64,
    //     text: String,
    //     options: Option<SendOptions>,
    // ) -> Result<Message, Box<dyn std::error::Error>>;
}
