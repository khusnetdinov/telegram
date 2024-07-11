use telegram_bots_api::api::enums::background_type::BackgroundType;
use telegram_bots_api::api::structs::chat_background::ChatBackground as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct ChatBackground {
    // #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: BackgroundType,
}
