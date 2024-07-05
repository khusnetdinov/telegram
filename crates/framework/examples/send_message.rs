pub struct SendMessage {
    pub chat_id: ChatUId, // signature
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,

    // signature
    pub parse_mode: Option<String>,

    // Options (7)
    pub message_thread_id: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
    pub business_connection_id: Option<String>,
    pub message_effect_id: Option<String>,
}
