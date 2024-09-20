use crate::feature::web_app::WebAppInfo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_query_results_button::InlineQueryResultsButton as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InlineQueryResultsButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}
