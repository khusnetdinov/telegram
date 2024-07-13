use telegram_bots_api::api::structs::web_app_data::WebAppData as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
