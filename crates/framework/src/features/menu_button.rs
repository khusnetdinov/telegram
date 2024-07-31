use crate::bots_api::BotsApi;
use crate::enums::menu_button::MenuButton as Receive;
use crate::enums::menu_button::MenuButton as Send;
use crate::traits::menu_button::MenuButton;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::get_chat_menu_button::GetChatMenuButton;
use telegram_bots_api::api::params::set_chat_menu_button::SetChatMenuButton;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl MenuButton for BotsApi {
    async fn get_chat_menu_button(
        &self,
        chat_id: i64,
    ) -> Result<Receive, Box<dyn std::error::Error>> {
        let params = GetChatMenuButton {
            chat_id: Some(ChatUId::from(chat_id)),
        };

        Ok(self.client.get_chat_menu_button(&params).await?.into())
    }

    async fn set_chat_menu_button(
        &self,
        chat_id: i64,
        menu_button: Option<Send>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatMenuButton {
            chat_id: Some(ChatUId::from(chat_id)),
            menu_button: menu_button.map(|inner| inner.into()),
        };

        Ok(self.client.set_chat_menu_button(&params).await?)
    }
}
