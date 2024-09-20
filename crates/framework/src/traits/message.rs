#[async_trait::async_trait]
pub trait Message {
    // async fn copy_message(
    //     &self,
    //     params: &CopyMessage,
    // ) -> Result<MessageId, Box<dyn std::error::Error>>;
    //
    // async fn copy_messages(
    //     &self,
    //     params: &CopyMessages,
    // ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>>;
    //
    // async fn delete_message(
    //     &self,
    //     params: &DeleteMessage,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn delete_messages(
    //     &self,
    //     params: &DeleteMessages,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn edit_message_caption(
    //     &self,
    //     params: &EditMessageCaption,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;
    //
    // async fn edit_message_live_location(
    //     &self,
    //     params: &EditMessageLiveLocation,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;
    //
    // async fn edit_message_media(
    //     &self,
    //     params: &EditMessageMedia,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;
    //
    // async fn edit_message_reply_markup(
    //     &self,
    //     params: &EditMessageReplyMarkup,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;
    //
    // async fn edit_message_text(
    //     &self,
    //     params: &EditMessageText,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;
    //
    // async fn forward_message(
    //     &self,
    //     params: &ForwardMessage,
    // ) -> Result<MessageId, Box<dyn std::error::Error>>;
    //
    // async fn forward_messages(
    //     &self,
    //     params: &ForwardMessages,
    // ) -> Result<Vec<MessageId>, Box<dyn std::error::Error>>;
    //
    // async fn set_message_reaction(
    //     &self,
    //     params: &SetMessageReaction,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn stop_message_live_location(
    //     &self,
    //     params: &StopMessageLiveLocation,
    // ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    // async fn send_message(
    //     &self,
    //     params: &SendMessage,
    // ) -> Result<Message, Box<dyn std::error::Error>>;
}
