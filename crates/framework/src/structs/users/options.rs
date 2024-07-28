use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#getuserprofilephotos>
/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
