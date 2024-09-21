use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
use crate::structs::updates::incoming_messages::contact::Contact as Send;
use crate::structs::updates::message::Message;
use crate::traits::features::contact::Contact;
use telegram_bots_api::api::params::send_contact::SendContact;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Contact for BotsApi {
    async fn send_contact(
        &self,
        chat_id: ChatUId,
        contact: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendContact {
                chat_id: chat_id.into(),
                phone_number: contact.phone_number,
                first_name: contact.first_name,
                last_name: contact.last_name,
                vcard: contact.vcard,
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters.map(|inner| inner.into()),
                reply_markup: options.reply_markup.map(|inner| inner.into()),
            }
        } else {
            SendContact {
                chat_id: chat_id.into(),
                phone_number: contact.phone_number,
                first_name: contact.first_name,
                last_name: contact.last_name,
                vcard: contact.vcard,
                ..Default::default()
            }
        };

        Ok(self.client.send_contact(&params).await?.into())
    }
}
