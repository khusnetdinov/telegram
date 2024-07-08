use crate::bots_api::BotsApi;
use crate::structs::bot_command::BotCommand;
use crate::traits::bots_apis::commands::Commands;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Commands for BotsApi {
    async fn commands(
        &self,
        params: (DeleteMyCommands, GetMyCommands, SetMyCommands),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (delete_params, _, set_params) = params;

        self.delete_commands(delete_params).await?;
        self.set_commands(set_params).await?;

        Ok(())
    }

    async fn delete_commands(
        &self,
        params: DeleteMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.delete_my_commands(&params).await?)
    }

    async fn get_commands(
        &self,
        params: GetMyCommands,
    ) -> Result<Vec<BotCommand>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get_my_commands(&params)
            .await?
            .iter()
            .map(|remote| remote.to_owned().into())
            .collect::<Vec<BotCommand>>())
    }

    async fn set_commands(
        &self,
        params: SetMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.set_my_commands(&params).await?)
    }
}
