pub struct SendDice {
    pub chat_id: ChatUId, // Signature
    pub emoji: Option<String>,

    // Options (7)
    pub message_thread_id: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
    pub business_connection_id: Option<String>,
    pub message_effect_id: Option<String>,
}
