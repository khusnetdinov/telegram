use crate::structs::chats::chat_administrator_rights::ChatAdministratorRights;
use crate::structs::my::bot_description::BotDescription;
use crate::structs::my::bot_name::BotName;
use crate::structs::my::bot_short_description::BotShortDescription;

#[async_trait::async_trait]
pub trait My {
    async fn get_my_description(
        &self,
        language_code: Option<String>,
    ) -> Result<BotDescription, Box<dyn std::error::Error>>;

    async fn get_my_name(
        &self,
        language_code: Option<String>,
    ) -> Result<BotName, Box<dyn std::error::Error>>;

    async fn get_my_short_description(
        &self,
        language_code: Option<String>,
    ) -> Result<BotShortDescription, Box<dyn std::error::Error>>;

    async fn set_my_description(
        &self,
        description: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_my_name(
        &self,
        name: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_my_short_description(
        &self,
        short_description: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_my_default_administrator_rights(
        &self,
        rights: Option<ChatAdministratorRights>,
        for_channels: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_my_default_administrator_rights(
        &self,
        for_channels: Option<bool>,
    ) -> Result<ChatAdministratorRights, Box<dyn std::error::Error>>;
}
