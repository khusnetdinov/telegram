use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::photo::Photo;
use telegram_bots_api::api::params::send_photo::SendPhoto;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Photo for BotsApi {
    async fn send_photo(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            parse_mode,
            has_spoiler,
            caption_entities,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendPhoto {
            chat_id: chat_id.into(),
            photo: file.into(),
            parse_mode,
            has_spoiler,
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            caption: options.caption,
            show_caption_above_media: options.show_caption_above_media,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_photo(&params).await?.into())
    }
}