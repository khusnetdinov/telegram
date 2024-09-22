use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
use crate::structs::updates::incoming_messages::venue::Venue as Send;
use crate::structs::updates::message::Message;
use crate::traits::features::geo::venue::Venue;
use telegram_bots_api::api::params::send_venue::SendVenue;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Venue for BotsApi {
    async fn send_venue(
        &self,
        chat_id: ChatUId,
        venue: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendVenue {
                chat_id: chat_id.into(),
                latitude: venue.location.latitude,
                longitude: venue.location.longitude,
                title: venue.title,
                address: venue.address,
                foursquare_id: venue.foursquare_id,
                foursquare_type: venue.foursquare_type,
                google_place_id: venue.google_place_id,
                google_place_type: venue.google_place_type,
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters.map(|inner| inner.into()),
                reply_markup: options.reply_markup.map(|inner| inner.into()),
            }
        } else {
            SendVenue {
                chat_id: chat_id.into(),
                latitude: venue.location.latitude,
                longitude: venue.location.longitude,
                title: venue.title,
                address: venue.address,
                foursquare_id: venue.foursquare_id,
                foursquare_type: venue.foursquare_type,
                google_place_id: venue.google_place_id,
                google_place_type: venue.google_place_type,
                ..Default::default()
            }
        };

        Ok(self.client.send_venue(&params).await?.into())
    }
}
