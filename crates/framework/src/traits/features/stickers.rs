use crate::enums::file_input::FileInput;
use crate::feature::bots_api::Options;
use crate::structs::file::File;
use crate::structs::input_file::InputFile;
use crate::structs::sticker::Sticker;
use crate::structs::stickers::input_sticker::InputSticker;
use crate::structs::stickers::mask_position::MaskPosition;
use crate::structs::stickers::options::Options as StickerOptions;
use crate::structs::stickers::sticker_set::StickerSet;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Stickers {
    async fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: String,
        title: String,
        stickers: Vec<InputSticker>,
        stickers_options: StickerOptions,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete_chat_sticker_set(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_chat_sticker_set(
        &self,
        chat_id: i64,
        sticker_set_name: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: String,
        custom_emoji_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_sticker_set(&self, name: String)
        -> Result<StickerSet, Box<dyn std::error::Error>>;

    async fn delete_sticker_from_set(
        &self,
        sticker: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_sticker_set_title(
        &self,
        name: String,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete_sticker_set(&self, name: String) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_sticker_set_thumbnail(
        &self,
        user_id: i64,
        name: String,
        format: String,
        thumbnail: Option<FileInput>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: String,
        old_sticker: String,
        sticker: InputSticker,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> Result<Sticker, Box<dyn std::error::Error>>;

    async fn get_forum_topic_icon_stickers(
        &self,
    ) -> Result<Vec<Sticker>, Box<dyn std::error::Error>>;

    async fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: String,
        sticker: InputSticker,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_sticker_emoji_list(
        &self,
        sticker: String,
        emoji_list: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_sticker_mask_position(
        &self,
        sticker: String,
        mask_position: Option<MaskPosition>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: InputFile,
        sticker_format: String,
    ) -> Result<File, Box<dyn std::error::Error>>;

    async fn set_sticker_keywords(
        &self,
        sticker: String,
        keywords: Option<Vec<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn send_sticker(
        &self,
        chat_id: i64,
        sticker: FileInput,
        emoji: Option<String>,
        options: Options,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
