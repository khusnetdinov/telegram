use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::input_media::InputMedia;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::media_group::MediaGroup;
use telegram_bots_api::api::params::send_media_group::SendMediaGroup;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl MediaGroup for BotsApi {
    async fn send_media_group(
        &self,
        chat_id: ChatUId,
        media: Vec<InputMedia>,
        options: MediaOptions,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let MediaOptions {
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            ..
        } = options;

        let params = SendMediaGroup {
            chat_id: chat_id.into(),
            media: media.iter().map(|inner| inner.to_owned().into()).collect(),
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
        };

        Ok(self
            .client
            .send_media_group(&params)
            .await?
            .iter()
            .map(|inner| inner.clone().into())
            .collect())
    }
}
