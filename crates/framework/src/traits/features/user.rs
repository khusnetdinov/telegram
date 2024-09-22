use crate::structs::users::user_profile_photos::UserProfilePhotos;

#[async_trait::async_trait]
pub trait User {
    async fn get_user_profile_photos(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<UserProfilePhotos, Box<dyn std::error::Error>>;
}
