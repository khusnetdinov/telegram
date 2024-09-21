use crate::enums::menu_button::MenuButton as Receive;
use crate::enums::menu_button::MenuButton as Send;

#[async_trait::async_trait]
pub trait MenuButton {
    async fn get_chat_menu_button(
        &self,
        chat_id: i64,
    ) -> Result<Receive, Box<dyn std::error::Error>>;

    async fn set_chat_menu_button(
        &self,
        chat_id: i64,
        menu_button: Option<Send>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
