use crate::bots_api::BotsApi;
use crate::enums::chat_action::ChatAction;
use crate::structs::options::Options;
use crate::traits::features::chat_actions::ChatActions;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_chat_action::SendChatAction;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl ChatActions for BotsApi {
    async fn send_chat_action(
        &self,
        chat_id: i64,
        action: ChatAction,
        options: Option<Options>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendChatAction {
                action: action.into(),
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                message_thread_id: options.message_thread_id,
            }
        } else {
            SendChatAction {
                action: action.into(),
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_chat_action(&params).await?)
    }
}
