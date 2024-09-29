use crate::enums::message_origin::MessageOrigin;
use crate::structs::chat::Chat;
use crate::structs::link_preview_options::LinkPreviewOptions;
use crate::structs::media::animation::Animation;
use crate::structs::media::audio::Audio;
use crate::structs::media::document::Document;
use crate::structs::media::paid_media::paid_media_info::PaidMediaInfo;
use crate::structs::media::photo_size::PhotoSize;
use crate::structs::media::story::Story;
use crate::structs::media::video::Video;
use crate::structs::media::video_note::VideoNote;
use crate::structs::media::voice::Voice;
use crate::structs::messages::message_id::MessageId;
use crate::structs::payments::invoice::Invoice;
use crate::structs::poll::Poll;
use crate::structs::sticker::Sticker;
use crate::structs::updates::incoming_messages::contact::Contact;
use crate::structs::updates::incoming_messages::dice::Dice;
use crate::structs::updates::incoming_messages::game::Game;
use crate::structs::updates::incoming_messages::geo::incoming_location::IncomingLocation;
use crate::structs::updates::incoming_messages::geo::incoming_venue::IncomingVenue;
use crate::structs::updates::incoming_messages::giveaway::Giveaway;
use crate::structs::updates::incoming_messages::giveaway_winners::GiveawayWinners;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::external_reply_info::ExternalReplyInfo as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ExternalReplyInfo {
    pub origin: MessageOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<IncomingLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<IncomingVenue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
}
