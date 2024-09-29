use crate::bots_api::BotsApi;
use crate::structs::chats::chat_administrator_rights::ChatAdministratorRights;
use crate::structs::my::bot_description::BotDescription;
use crate::structs::my::bot_name::BotName;
use crate::structs::my::bot_short_description::BotShortDescription;
use crate::traits::features::my::My;
use telegram_bots_api::api::params::get_my_default_administrator_rights::GetMyDefaultAdministratorRights;
use telegram_bots_api::api::params::get_my_description::GetMyDescription;
use telegram_bots_api::api::params::get_my_name::GetMyName;
use telegram_bots_api::api::params::get_my_short_description::GetMyShortDescription;
use telegram_bots_api::api::params::set_my_default_administrator_rights::SetMyDefaultAdministratorRights;
use telegram_bots_api::api::params::set_my_description::SetMyDescription;
use telegram_bots_api::api::params::set_my_name::SetMyName;
use telegram_bots_api::api::params::set_my_short_description::SetMyShortDescription;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl My for BotsApi {
    async fn get_my_description(
        &self,
        language_code: Option<String>,
    ) -> Result<BotDescription, Box<dyn std::error::Error>> {
        let params = GetMyDescription { language_code };

        Ok(self.client.get_my_description(&params).await?.into())
    }

    async fn get_my_name(
        &self,
        language_code: Option<String>,
    ) -> Result<BotName, Box<dyn std::error::Error>> {
        let params = GetMyName { language_code };

        Ok(self.client.get_my_name(&params).await?.into())
    }

    async fn get_my_short_description(
        &self,
        language_code: Option<String>,
    ) -> Result<BotShortDescription, Box<dyn std::error::Error>> {
        let params = GetMyShortDescription { language_code };

        Ok(self.client.get_my_short_description(&params).await?.into())
    }

    async fn set_my_description(
        &self,
        description: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetMyDescription {
            description,
            language_code,
        };

        Ok(self.client.set_my_description(&params).await?)
    }

    async fn set_my_name(
        &self,
        name: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetMyName {
            name,
            language_code,
        };

        Ok(self.client.set_my_name(&params).await?)
    }

    async fn set_my_short_description(
        &self,
        short_description: Option<String>,
        language_code: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetMyShortDescription {
            short_description,
            language_code,
        };

        Ok(self.client.set_my_short_description(&params).await?)
    }

    async fn set_my_default_administrator_rights(
        &self,
        rights: Option<ChatAdministratorRights>,
        for_channels: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetMyDefaultAdministratorRights {
            rights: rights.map(|inner| inner.into()),
            for_channels,
        };

        Ok(self
            .client
            .set_my_default_administrator_rights(&params)
            .await?)
    }

    async fn get_my_default_administrator_rights(
        &self,
        for_channels: Option<bool>,
    ) -> Result<ChatAdministratorRights, Box<dyn std::error::Error>> {
        let params = GetMyDefaultAdministratorRights { for_channels };

        Ok(self
            .client
            .get_my_default_administrator_rights(&params)
            .await?
            .into())
    }
}
