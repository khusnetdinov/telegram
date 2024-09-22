use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::features::geo::location::Location;
use std::error::Error;
use telegram_bots_api::api::params::send_location::SendLocation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Location for BotsApi {
    async fn send_location(
        &self,
        chat_id: ChatUId,
        location: crate::structs::updates::incoming_messages::location::Location,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn Error>> {
        let params = if let Some(options) = options {
            SendLocation {
                chat_id: chat_id.into(),
                latitude: location.latitude,
                longitude: location.longitude,
                horizontal_accuracy: location.horizontal_accuracy,
                live_period: location.live_period,
                heading: location.heading,
                proximity_alert_radius: location.proximity_alert_radius,
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters.map(|inner| inner.into()),
                reply_markup: options.reply_markup.map(|inner| inner.into()),
            }
        } else {
            SendLocation {
                chat_id: chat_id.into(),
                latitude: location.latitude,
                longitude: location.longitude,
                horizontal_accuracy: location.horizontal_accuracy,
                live_period: location.live_period,
                heading: location.heading,
                proximity_alert_radius: location.proximity_alert_radius,
                ..Default::default()
            }
        };

        Ok(self.client.send_location(&params).await?.into())
    }
}
