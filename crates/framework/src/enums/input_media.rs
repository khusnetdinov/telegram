use crate::structs::media::input_media_animation::InputMediaAnimation;
use crate::structs::media::input_media_audio::InputMediaAudio;
use crate::structs::media::input_media_document::InputMediaDocument;
use crate::structs::media::input_media_photo::InputMediaPhoto;
use crate::structs::media::input_media_video::InputMediaVideo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::input_media::InputMedia as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl Default for InputMedia {
    fn default() -> Self {
        Self::Animation(InputMediaAnimation {
            ..Default::default()
        })
    }
}

// impl From<InputMedia> for Remote {
//     fn from(value: InputMedia) -> Self {
//         match value {
//             InputMedia::Animation(media) => Self::Animation(media.into()),
//             InputMedia::Document(media) => Self::Document(media.into()),
//             InputMedia::Audio(media) => Self::Audio(media.into()),
//             InputMedia::Photo(media) => Self::Photo(media.into()),
//             InputMedia::Video(media) => Self::Video(media.into()),
//         }
//     }
// }
