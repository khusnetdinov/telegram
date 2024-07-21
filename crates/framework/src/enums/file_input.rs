use crate::structs::input_file::InputFile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use telegram_bots_api::api::enums::file_input::FileInput as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FileInput {
    InputFile(InputFile),
    String(String),
}

impl Default for FileInput {
    fn default() -> Self {
        Self::String(String::from(""))
    }
}

impl From<PathBuf> for FileInput {
    fn from(path: PathBuf) -> Self {
        let input_file = InputFile { path };

        Self::InputFile(input_file)
    }
}

impl From<InputFile> for FileInput {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

impl From<String> for FileInput {
    fn from(file: String) -> Self {
        Self::String(file)
    }
}

impl From<Remote> for FileInput {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::InputFile(input_file) => Self::InputFile(input_file.into()),
            Remote::String(string) => Self::String(string),
        }
    }
}

impl From<FileInput> for Remote {
    fn from(value: FileInput) -> Self {
        match value {
            FileInput::InputFile(input_file) => Self::InputFile(input_file.into()),
            FileInput::String(string) => Self::String(string),
        }
    }
}
