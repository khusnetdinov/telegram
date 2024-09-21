use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectedWebsite {
    pub connected_website: String,
}

impl From<IncomingMessage> for ConnectedWebsite {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            connected_website, ..
        } = remote;

        Self {
            connected_website: connected_website.unwrap(),
        }
    }
}
