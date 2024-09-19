use crate::feature::web_app::WebAppInfo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_web_app::MenuButtonWebApp as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct MenuButtonWebApp {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub text: String,
    pub web_app: WebAppInfo,
}
