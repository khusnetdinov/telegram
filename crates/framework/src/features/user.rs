use crate::bots_api::BotsApi;
use crate::structs::users::options::Options as UserOptions;
use crate::structs::users::user_profile_photos::UserProfilePhotos;
use crate::traits::user::User;
use telegram_bots_api::api::params::get_user_profile_photos::GetUserProfilePhotos;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl User for BotsApi {
    async fn get_user_profile_photos(
        &self,
        user_id: i64,
        user_options: UserOptions,
    ) -> Result<UserProfilePhotos, Box<dyn std::error::Error>> {
        let UserOptions { limit, offset } = user_options;

        let params = GetUserProfilePhotos {
            user_id,
            limit,
            offset,
        };

        Ok(self.client.get_user_profile_photos(&params).await?.into())
    }
}
