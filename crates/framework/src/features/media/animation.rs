use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::animation::Animation;
use telegram_bots_api::api::params::send_animation::SendAnimation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Animation for BotsApi {
    async fn send_animation(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            parse_mode,
            has_spoiler,
            caption_entities,
            caption,
            show_caption_above_media,
            height,
            width,
            duration,
            thumbnail,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendAnimation {
            chat_id: chat_id.into(),
            animation: file.into(),
            height,
            width,
            duration,
            thumbnail: thumbnail.map(|inner| inner.into()),
            parse_mode,
            has_spoiler,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            caption,
            show_caption_above_media,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_animation(&params).await?.into())
    }
}
