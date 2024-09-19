use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_commands::MenuButtonCommands as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct MenuButtonCommands {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
