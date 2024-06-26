use crate::structs::bot_command::BotCommand;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;

#[async_trait::async_trait]
pub trait Commander {
    async fn commands(
        &self,
        params: (DeleteMyCommands, GetMyCommands, SetMyCommands),
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn delete_commands(
        &self,
        params: DeleteMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_commands(
        &self,
        params: GetMyCommands,
    ) -> Result<Vec<BotCommand>, Box<dyn std::error::Error>>;

    async fn set_commands(&self, params: SetMyCommands)
        -> Result<bool, Box<dyn std::error::Error>>;
}
