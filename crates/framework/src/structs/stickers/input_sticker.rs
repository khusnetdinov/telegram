use crate::enums::file_input::FileInput;
use crate::structs::stickers::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_sticker::InputSticker as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InputSticker {
    pub sticker: FileInput,
    pub emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    pub format: String,
}
