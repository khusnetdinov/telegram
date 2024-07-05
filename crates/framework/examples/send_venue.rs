pub struct SendVenue {
    pub chat_id: ChatUId, // signature

    pub latitude: f64,  // Location
    pub longitude: f64, // Location
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,

    // Options (7)
    pub message_thread_id: Option<i64>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
    pub business_connection_id: Option<String>,
    pub message_effect_id: Option<String>,
}
