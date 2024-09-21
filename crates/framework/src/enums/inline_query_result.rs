use crate::structs::inline_query_results::inline_query_result_article::InlineQueryResultArticle;
use crate::structs::inline_query_results::inline_query_result_audio::InlineQueryResultAudio;
use crate::structs::inline_query_results::inline_query_result_cached_audio::InlineQueryResultCachedAudio;
use crate::structs::inline_query_results::inline_query_result_cached_document::InlineQueryResultCachedDocument;
use crate::structs::inline_query_results::inline_query_result_cached_gif::InlineQueryResultCachedGif;
use crate::structs::inline_query_results::inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
use crate::structs::inline_query_results::inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
use crate::structs::inline_query_results::inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
use crate::structs::inline_query_results::inline_query_result_cached_video::InlineQueryResultCachedVideo;
use crate::structs::inline_query_results::inline_query_result_cached_voice::InlineQueryResultCachedVoice;
use crate::structs::inline_query_results::inline_query_result_contact::InlineQueryResultContact;
use crate::structs::inline_query_results::inline_query_result_document::InlineQueryResultDocument;
use crate::structs::inline_query_results::inline_query_result_game::InlineQueryResultGame;
use crate::structs::inline_query_results::inline_query_result_gif::InlineQueryResultGif;
use crate::structs::inline_query_results::inline_query_result_location::InlineQueryResultLocation;
use crate::structs::inline_query_results::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
use crate::structs::inline_query_results::inline_query_result_photo::InlineQueryResultPhoto;
use crate::structs::inline_query_results::inline_query_result_venue::InlineQueryResultVenue;
use crate::structs::inline_query_results::inline_query_result_video::InlineQueryResultVideo;
use crate::structs::inline_query_results::inline_query_result_voice::InlineQueryResultVoice;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::inline_query_result::InlineQueryResult as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum InlineQueryResult {
    CachedAudio(InlineQueryResultCachedAudio),
    CachedDocument(InlineQueryResultCachedDocument),
    CachedGif(InlineQueryResultCachedGif),
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    CachedPhoto(InlineQueryResultCachedPhoto),
    CachedSticker(InlineQueryResultCachedSticker),
    CachedVideo(InlineQueryResultCachedVideo),
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Document(InlineQueryResultDocument),
    Game(InlineQueryResultGame),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

impl Default for InlineQueryResult {
    fn default() -> Self {
        Self::CachedAudio(InlineQueryResultCachedAudio {
            ..Default::default()
        })
    }
}
