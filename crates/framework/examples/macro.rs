use telegram_bots_api::api::structs::user_profile_photos::UserProfilePhotos as Remote;
use telegram_framework::feature::animation::PhotoSize;
use telegram_framework::structs::user::User;
use telegram_macros::FromRemoteStruct;

#[derive(FromRemoteStruct)]
pub struct UserProfilePhotos {
    field1: String,
    field2: User,
    field3: Option<String>,
    field4: Option<User>,
    field5: Vec<String>,
    field6: Vec<User>,
    filed7: Box<String>,
    field8: Box<User>,
    field9: Option<Vec<String>>,
    field10: Option<Vec<User>>,
    field11: Option<Box<String>>,
    field12: Option<Box<User>>,
    filed13: Vec<Vec<PhotoSize>>,
}
