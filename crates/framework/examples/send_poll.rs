// pub struct SendPoll {
//     #[serde(rename(serialize = "type", deserialize = "type"))]
//     pub kind: Option<String>,
//     pub chat_id: ChatUId, // Signatur
//     pub question: String,
//     pub options: Vec<InputPollOption>,
//     pub allows_multiple_answers: Option<bool>,
//     pub is_closed: Option<bool>,
//     pub is_anonymous: Option<bool>,
//     pub close_date: Option<i64>,
//     pub correct_option_id: Option<i64>,
//     pub explanation: Option<String>,
//     pub explanation_entities: Option<Vec<MessageEntity>>,
//     pub open_period: Option<i64>,

//     // Struct
//     pub explanation_parse_mode: Option<String>,
//     pub question_parse_mode: Option<String>,
//     pub question_entities: Option<MessageEntity>,

//     // Options (7)
//     pub message_thread_id: Option<i64>,
//     pub disable_notification: Option<bool>,
//     pub protect_content: Option<bool>,
//     pub reply_parameters: Option<ReplyParameters>,
//     pub reply_markup: Option<ReplyMarkup>,
//     pub business_connection_id: Option<String>,
//     pub message_effect_id: Option<String>,
// }
