use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::message::Message;
use crate::traits::features::media::audio::Audio;
use telegram_bots_api::api::params::send_audio::SendAudio;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Audio for BotsApi {
    async fn send_audio(
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
            duration,
            thumbnail,
            performer,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendAudio {
            chat_id: chat_id.into(),
            audio: file.into(),
            duration,
            thumbnail: thumbnail.map(|inner| inner.into()),
            parse_mode,
            has_spoiler,
            performer,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            caption,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_audio(&params).await?.into())
    }
}
