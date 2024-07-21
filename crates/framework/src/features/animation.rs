use crate::bots_api::BotsApi;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::animation::Animation;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_animation::SendAnimation;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Animation for BotsApi {
    async fn send_animation(
        &self,
        chat_id: i64,
        animation: FileInput,
        media_options: MediaOptions,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            parse_mode,
            has_spoiler,
            caption_entities,
            height,
            width,
            duration,
            thumbnail,
            ..
        } = media_options;

        let params = if let Some(options) = options {
            SendAnimation {
                chat_id: ChatUId::from(chat_id),
                animation: animation.into(),
                height,
                width,
                duration,
                thumbnail,
                parse_mode,
                has_spoiler,
                // TODO: #[remote(option, map, into)]
                caption_entities: caption_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                caption: options.caption,
                show_caption_above_media: options.show_caption_above_media,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                business_connection_id: options.business_connection_id,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendAnimation {
                chat_id: ChatUId::from(chat_id),
                animation: animation.into(),
                height,
                width,
                duration,
                thumbnail,
                parse_mode,
                has_spoiler,
                // TODO: #[remote(option, map, into)]
                caption_entities: caption_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                ..Default::default()
            }
        };

        Ok(self.client.send_animation(&params).await?.into())
    }
}
