use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::geo::location::Location as Send;
use crate::structs::geo::options::Options as GeoOptions;
use crate::structs::message::Message;
use crate::traits::features::geo::location::Location;
use telegram_bots_api::api::params::send_location::SendLocation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Location for BotsApi {
    async fn send_location(
        &self,
        chat_id: ChatUId,
        location: Send,
        options: GeoOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let GeoOptions {
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendLocation {
            chat_id: chat_id.into(),
            latitude: location.latitude,
            longitude: location.longitude,
            heading: location.heading,
            horizontal_accuracy: location.horizontal_accuracy,
            live_period: location.live_period,
            proximity_alert_radius: location.proximity_alert_radius,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_location(&params).await?.into())
    }
}
