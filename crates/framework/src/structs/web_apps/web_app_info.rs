use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::web_app_info::WebAppInfo as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct WebAppInfo {
    pub url: String,
}
