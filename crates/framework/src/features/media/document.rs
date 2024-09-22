use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::document::Document;
use telegram_bots_api::api::params::send_document::SendDocument;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Document for BotsApi {
    async fn send_document(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            parse_mode,
            caption_entities,
            caption,
            thumbnail,
            disable_content_type_detection,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendDocument {
            chat_id: chat_id.into(),
            document: file.into(),
            thumbnail: thumbnail.map(|inner| inner.into()),
            parse_mode,
            disable_content_type_detection,
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

        Ok(self.client.send_document(&params).await?.into())
    }
}
