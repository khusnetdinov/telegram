use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectedWebsiteMessage {
    pub connected_website: String,
}

impl From<Inner> for ConnectedWebsiteMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            connected_website, ..
        } = inner;

        Self {
            connected_website: connected_website.unwrap(),
        }
    }
}
