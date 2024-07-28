use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::user_profile_photos::UserProfilePhotos as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

impl From<Remote> for UserProfilePhotos {
    fn from(remote: Remote) -> Self {
        Self {
            total_count: remote.total_count,
            // TODO: #[remote(map, map, into)]
            photos: remote
                .photos
                .iter()
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect())
                .collect(),
        }
    }
}
