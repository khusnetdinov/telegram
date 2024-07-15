use crate::bots_api::BotsApi;
use crate::feature::bots_api::Options;
use crate::structs::updates::message::Message;
use crate::traits::location::Location;
use std::error::Error;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_location::SendLocation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]

impl Location for BotsApi {
    async fn send_location(
        &self,
        chat_id: i64,
        location: crate::structs::location::Location,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn Error>> {
        let params = if let Some(options) = options {
            SendLocation {
                chat_id: ChatUId::from(chat_id),
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
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendLocation {
                chat_id: ChatUId::from(chat_id),
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
