use telegram_bots_api::api::structs::photo_size::PhotoSize;
use telegram_bots_api::api::structs::shared_user::SharedUser as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct SharedUser {
    pub user_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
