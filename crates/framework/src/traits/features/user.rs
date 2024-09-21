use crate::structs::users::options::Options as UserOptions;
use crate::structs::users::user_profile_photos::UserProfilePhotos;

#[async_trait::async_trait]
pub trait User {
    async fn get_user_profile_photos(
        &self,
        user_id: i64,
        user_option: UserOptions,
    ) -> Result<UserProfilePhotos, Box<dyn std::error::Error>>;
}
