use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use telegram_bots_api::api::structs::input_file::InputFile as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputFile {
    pub path: PathBuf,
}

impl From<Remote> for InputFile {
    fn from(remote: Remote) -> Self {
        Self { path: remote.path }
    }
}

impl From<InputFile> for Remote {
    fn from(value: InputFile) -> Self {
        Self { path: value.path }
    }
}
