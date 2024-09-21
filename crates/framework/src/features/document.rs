use crate::bots_api::BotsApi;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::features::document::Document;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_document::SendDocument;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Document for BotsApi {
    async fn send_document(
        &self,
        chat_id: i64,
        file: FileInput,
        media_options: MediaOptions,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            parse_mode,
            caption_entities,
            thumbnail,
            disable_content_type_detection,
            ..
        } = media_options;

        let params = if let Some(options) = options {
            SendDocument {
                chat_id: ChatUId::from(chat_id),
                document: file.into(),
                thumbnail,
                parse_mode,
                disable_content_type_detection,
                // TODO: #[remote(option, map, into)]
                caption_entities: caption_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                caption: options.caption,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                business_connection_id: options.business_connection_id,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendDocument {
                chat_id: ChatUId::from(chat_id),
                document: file.into(),
                thumbnail,
                parse_mode,
                disable_content_type_detection,
                // TODO: #[remote(option, map, into)]
                caption_entities: caption_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                ..Default::default()
            }
        };

        Ok(self.client.send_document(&params).await?.into())
    }
}
