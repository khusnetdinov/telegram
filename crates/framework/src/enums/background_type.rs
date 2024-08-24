use crate::structs::background_types::background_type_chat_theme::BackgroundTypeChatTheme;
use crate::structs::background_types::background_type_fill::BackgroundTypeFill;
use crate::structs::background_types::background_type_pattern::BackgroundTypePattern;
use crate::structs::background_types::background_type_wallpaper::BackgroundTypeWallpaper;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::background_type::BackgroundType as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

impl Default for BackgroundType {
    fn default() -> Self {
        Self::Fill(BackgroundTypeFill {
            ..Default::default()
        })
    }
}
