use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::feature::animation::MediaOptions;
use crate::feature::bots_api::Options;
use crate::structs::updates::message::Message;
use crate::traits::features::video::Video;
use telegram_bots_api::api::params::send_video::SendVideo;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Video for BotsApi {
    async fn send_video(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
        options: Option<Options>,
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
            ..
        } = media_options;

        let params = if let Some(options) = options {
            SendVideo {
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
                // TODO: #[remote(option, map, into)]
                caption_entities: caption_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                show_caption_above_media: options.show_caption_above_media,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                business_connection_id: options.business_connection_id,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters.map(|inner| inner.into()),
                reply_markup: options.reply_markup.map(|inner| inner.into()),
            }
        } else {
            SendVideo {
                chat_id: chat_id.into(),
                video: file.into(),
                duration,
                width,
                height,
                thumbnail: thumbnail.map(|inner| inner.into()),
                has_spoiler,
                supports_streaming,
                parse_mode,
                ..Default::default()
            }
        };

        Ok(self.client.send_video(&params).await?.into())
    }
}
