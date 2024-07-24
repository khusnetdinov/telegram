use telegram_bots_api::api::structs::web_app_info::WebAppInfo as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct WebAppInfo {
    pub url: String,
}
