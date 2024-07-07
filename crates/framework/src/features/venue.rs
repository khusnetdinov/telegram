impl Venue for BotsApi {
    async fn send_venue(
        &self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        options: Option<SendOptions>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendVenue {
                latitude,
                longitude,
                title,
                address,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
                foursquare_id: options.foursquare_id,
                foursquare_type: options.foursquare_type,
                google_place_id: options.google_place_id,
                google_place_type: options.google_place_type,
            }
        } else {
            SendVenue {
                latitude,
                longitude,
                title,
                address,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_venue(&params).await?.into())
    }
}
