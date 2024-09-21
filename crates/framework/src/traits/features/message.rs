use crate::enums::chat_uid::ChatUId;
use crate::enums::input_media::InputMedia;
use crate::enums::message_result::MessageResult;
use crate::enums::reaction_type::ReactionType;
use crate::structs::messages::message_id::MessageId;
use crate::structs::messages::options::Options as MessageOptions;
use crate::structs::updates::message::Message as Response;

#[async_trait::async_trait]
pub trait Message {
    async fn copy_message(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageId, Box<dyn std::error::Error>>;

    async fn copy_messages(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_ids: Vec<MessageId>,
        message_options: MessageOptions,
    ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>>;

    async fn delete_message(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete_messages(
        &self,
        chat_id: ChatUId,
        message_ids: Vec<i64>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    // TODO: Required if inline_message_id is not specified
    async fn edit_message_caption(
        &self,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    // TODO: Required if inline_message_id is not specified
    async fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
        live_period: Option<i64>,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    // TODO: Required if inline_message_id is not specified
    async fn edit_message_media(
        &self,
        media: InputMedia,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    // TODO: Required if inline_message_id is not specified
    async fn edit_message_reply_markup(
        &self,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    // TODO: Required if inline_message_id is not specified
    async fn edit_message_text(
        &self,
        text: String,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    async fn forward_message(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_id: MessageId,
        message_options: MessageOptions,
    ) -> Result<MessageId, Box<dyn std::error::Error>>;

    async fn forward_messages(
        &self,
        chat_id: ChatUId,
        from_chat_id: ChatUId,
        message_ids: Vec<MessageId>,
        message_options: MessageOptions,
    ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>>;

    async fn set_message_reaction(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        reaction: Option<Vec<ReactionType>>,
        is_big: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn stop_message_live_location(
        &self,
        message_options: MessageOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    async fn send_message(
        &self,
        chat_id: ChatUId,
        text: String,
        message_options: MessageOptions,
    ) -> Result<Response, Box<dyn std::error::Error>>;
}
