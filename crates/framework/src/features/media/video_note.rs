use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;
use crate::traits::features::media::video_note::VideoNote;
use telegram_bots_api::api::params::send_video_note::SendVideoNote;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl VideoNote for BotsApi {
    async fn send_video_note(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            duration,
            length,
            thumbnail,
            business_connection_id,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendVideoNote {
            chat_id: chat_id.into(),
            video_note: file.into(),
            duration,
            length,
            thumbnail: thumbnail.map(|inner| inner.into()),
            disable_notification: options.disable_notification,
            protect_content: options.protect_content,
            business_connection_id,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_video_note(&params).await?.into())
    }
}
