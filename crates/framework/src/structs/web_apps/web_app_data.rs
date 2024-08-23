use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::web_app_data::WebAppData as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
