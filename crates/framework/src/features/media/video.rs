use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::video::Video;
use telegram_bots_api::api::params::send_video::SendVideo;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Video for BotsApi {
    async fn send_video(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            caption_entities,
            duration,
            thumbnail,
            width,
            height,
            parse_mode,
            has_spoiler,
            supports_streaming,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendVideo {
            chat_id: chat_id.into(),
            video: file.into(),
            duration,
            width,
            height,
            thumbnail: thumbnail.map(|inner| inner.into()),
            has_spoiler,
            supports_streaming,
            parse_mode,
            caption: options.caption,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media: options.show_caption_above_media,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_video(&params).await?.into())
    }
}
