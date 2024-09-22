use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::file::File;
use crate::structs::input_file::InputFile;
use crate::structs::options::Options;
use crate::structs::sticker::Sticker;
use crate::structs::stickers::input_sticker::InputSticker;
use crate::structs::stickers::mask_position::MaskPosition;
use crate::structs::stickers::options::Options as StickerOptions;
use crate::structs::stickers::sticker_set::StickerSet;
use crate::structs::updates::message::Message;
use crate::traits::features::stickers::Stickers;
use telegram_bots_api::api::params::add_sticker_to_set::AddStickerToSet;
use telegram_bots_api::api::params::create_new_sticker_set::CreateNewStickerSet;
use telegram_bots_api::api::params::delete_chat_sticker_set::DeleteChatStickerSet;
use telegram_bots_api::api::params::delete_sticker_from_set::DeleteStickerFromSet;
use telegram_bots_api::api::params::delete_sticker_set::DeleteStickerSet;
use telegram_bots_api::api::params::get_custom_emoji_stickers::GetCustomEmojiStickers;
use telegram_bots_api::api::params::get_forum_topic_icon_stickers::GetForumTopicIconStickers;
use telegram_bots_api::api::params::get_sticker_set::GetStickerSet;
use telegram_bots_api::api::params::replace_sticker_in_set::ReplaceStickerInSet;
use telegram_bots_api::api::params::send_sticker::SendSticker;
use telegram_bots_api::api::params::set_chat_sticker_set::SetChatStickerSet;
use telegram_bots_api::api::params::set_custom_emoji_sticker_set_thumbnail::SetCustomEmojiStickerSetThumbnail;
use telegram_bots_api::api::params::set_sticker_emoji_list::SetStickerEmojiList;
use telegram_bots_api::api::params::set_sticker_keywords::SetStickerKeywords;
use telegram_bots_api::api::params::set_sticker_mask_position::SetStickerMaskPosition;
use telegram_bots_api::api::params::set_sticker_position_in_set::SetStickerPositionInSet;
use telegram_bots_api::api::params::set_sticker_set_thumbnail::SetStickerSetThumbnail;
use telegram_bots_api::api::params::set_sticker_set_title::SetStickerSetTitle;
use telegram_bots_api::api::params::upload_sticker_file::UploadStickerFile;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Stickers for BotsApi {
    async fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: String,
        title: String,
        stickers: Vec<InputSticker>,
        stickers_options: StickerOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let StickerOptions {
            sticker_type,
            needs_repainting,
            ..
        } = stickers_options;

        let params = CreateNewStickerSet {
            user_id,
            name,
            title,
            stickers: stickers
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            sticker_type,
            needs_repainting,
        };

        Ok(self.client.create_new_sticker_set(&params).await?)
    }

    async fn delete_chat_sticker_set(
        &self,
        chat_id: ChatUId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteChatStickerSet {
            chat_id: chat_id.into(),
        };

        Ok(self.client.delete_chat_sticker_set(&params).await?)
    }

    async fn set_chat_sticker_set(
        &self,
        chat_id: ChatUId,
        sticker_set_name: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatStickerSet {
            chat_id: chat_id.into(),
            sticker_set_name,
        };

        Ok(self.client.set_chat_sticker_set(&params).await?)
    }

    async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: String,
        custom_emoji_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetCustomEmojiStickerSetThumbnail {
            name,
            custom_emoji_id,
        };

        Ok(self
            .client
            .set_custom_emoji_sticker_set_thumbnail(&params)
            .await?)
    }

    async fn get_sticker_set(
        &self,
        name: String,
    ) -> Result<StickerSet, Box<dyn std::error::Error>> {
        let params = GetStickerSet { name };

        Ok(self.client.get_sticker_set(&params).await?.into())
    }

    async fn delete_sticker_from_set(
        &self,
        sticker: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteStickerFromSet { sticker };

        Ok(self.client.delete_sticker_from_set(&params).await?)
    }

    async fn set_sticker_set_title(
        &self,
        name: String,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerSetTitle { name, title };

        Ok(self.client.set_sticker_set_title(&params).await?)
    }

    async fn delete_sticker_set(&self, name: String) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteStickerSet { name };

        Ok(self.client.delete_sticker_set(&params).await?)
    }

    async fn set_sticker_set_thumbnail(
        &self,
        user_id: i64,
        name: String,
        format: String,
        thumbnail: Option<FileInput>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerSetThumbnail {
            user_id,
            name,
            format,
            thumbnail: thumbnail.map(|inner| inner.into()),
        };

        Ok(self.client.set_sticker_set_thumbnail(&params).await?)
    }

    async fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: String,
        old_sticker: String,
        sticker: InputSticker,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = ReplaceStickerInSet {
            user_id,
            name,
            old_sticker,
            sticker: sticker.into(),
        };

        Ok(self.client.replace_sticker_in_set(&params).await?)
    }

    async fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerPositionInSet { sticker, position };

        Ok(self.client.set_sticker_position_in_set(&params).await?)
    }

    async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> Result<Sticker, Box<dyn std::error::Error>> {
        let params = GetCustomEmojiStickers { custom_emoji_ids };

        Ok(self.client.get_custom_emoji_stickers(&params).await?.into())
    }

    async fn get_forum_topic_icon_stickers(
        &self,
    ) -> Result<Vec<Sticker>, Box<dyn std::error::Error>> {
        let params = GetForumTopicIconStickers {};

        Ok(self
            .client
            .get_forum_topic_icon_stickers(&params)
            .await?
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect())
    }

    async fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: String,
        sticker: InputSticker,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = AddStickerToSet {
            user_id,
            name,
            sticker: sticker.into(),
        };

        Ok(self.client.add_sticker_to_set(&params).await?)
    }

    async fn set_sticker_emoji_list(
        &self,
        sticker: String,
        emoji_list: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerEmojiList {
            sticker,
            emoji_list,
        };

        Ok(self.client.set_sticker_emoji_list(&params).await?)
    }

    async fn set_sticker_mask_position(
        &self,
        sticker: String,
        mask_position: Option<MaskPosition>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerMaskPosition {
            sticker,
            mask_position: mask_position.map(|inner| inner.to_owned().into()),
        };

        Ok(self.client.set_sticker_mask_position(&params).await?)
    }

    async fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: InputFile,
        sticker_format: String,
    ) -> Result<File, Box<dyn std::error::Error>> {
        let params = UploadStickerFile {
            user_id,
            sticker: sticker.into(),
            sticker_format,
        };

        Ok(self.client.upload_sticker_file(&params).await?.into())
    }

    async fn set_sticker_keywords(
        &self,
        sticker: String,
        keywords: Option<Vec<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetStickerKeywords { sticker, keywords };

        Ok(self.client.set_sticker_keywords(&params).await?)
    }

    async fn send_sticker(
        &self,
        chat_id: ChatUId,
        sticker: FileInput,
        emoji: Option<String>,
        options: Options,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let Options {
            message_thread_id,
            disable_notification,
            protect_content,
            reply_parameters,
            reply_markup,
            business_connection_id,
            message_effect_id,
            ..
        } = options;

        let params = SendSticker {
            chat_id: chat_id.into(),
            sticker: sticker.into(),
            emoji,
            message_thread_id,
            disable_notification,
            protect_content,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
            business_connection_id,
            message_effect_id,
        };

        Ok(self.client.send_sticker(&params).await?.into())
    }
}
