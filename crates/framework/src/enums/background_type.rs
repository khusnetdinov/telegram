use crate::structs::background_types::background_type_chat_theme::BackgroundTypeChatTheme;
use crate::structs::background_types::background_type_fill::BackgroundTypeFill;
use crate::structs::background_types::background_type_pattern::BackgroundTypePattern;
use crate::structs::background_types::background_type_wallpaper::BackgroundTypeWallpaper;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::background_type::BackgroundType as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

impl From<Remote> for BackgroundType {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Fill(fill) => Self::Fill(fill.into()),
            Remote::Wallpaper(wallpaper) => Self::Wallpaper(wallpaper.into()),
            Remote::Pattern(pattern) => Self::Pattern(pattern.into()),
            Remote::ChatTheme(chat_theme) => Self::ChatTheme(chat_theme.into()),
        }
    }
}

impl Default for BackgroundType {
    fn default() -> Self {
        Self::Fill(BackgroundTypeFill {
            ..Default::default()
        })
    }
}
