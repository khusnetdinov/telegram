use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::contact::Contact as Send;
use crate::structs::contacts::options::Options as ContactOptions;
use crate::structs::message::Message;
use crate::traits::features::contact::Contact;
use telegram_bots_api::api::params::send_contact::SendContact;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Contact for BotsApi {
    async fn send_contact(
        &self,
        chat_id: ChatUId,
        contact: Send,
        options: ContactOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let ContactOptions {
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendContact {
            chat_id: chat_id.into(),
            phone_number: contact.phone_number,
            first_name: contact.first_name,
            last_name: contact.last_name,
            vcard: contact.vcard,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_contact(&params).await?.into())
    }
}
