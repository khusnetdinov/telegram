use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use telegram_bots_api::api::structs::input_file::InputFile as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct InputFile {
    pub path: PathBuf,
}

impl From<InputFile> for Remote {
    fn from(value: InputFile) -> Self {
        Self { path: value.path }
    }
}
