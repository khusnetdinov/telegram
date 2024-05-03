use crate::structs::command::Command;
use telegram_bots_api::api::enums::bot_command_scopes::BotCommandScopes;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Commands {
    pub commands: Vec<Command>,
    pub language_code: Option<String>,
    pub scope: Option<BotCommandScopes>,
}
