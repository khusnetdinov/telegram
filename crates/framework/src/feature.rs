pub mod animation {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::animation::Animation;
    pub use crate::structs::media::document::Document;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::messages::animation::Animation as AnimationMessage;
    pub use crate::traits::animation::Animation as AnimationTrait;
}

pub mod audio {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::audio::Audio;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::messages::audio::Audio as AudioMessage;
    pub use crate::traits::audio::Audio as AudioTrait;
}

pub mod bots_api {
    pub use crate::bots_api::BotsApi;
    pub use crate::enums::messages::Messages;
    pub use crate::enums::updates::Updates;
    pub use crate::storages::memory::MemoryStorage;
    pub use crate::structs::options::Options;
    pub use crate::structs::update::Update;
    pub use crate::traits::kind_dispatcher::KindDispatcher as KindDispatcherTrait;
    pub use std::fmt::Debug;
    pub use std::path::PathBuf;
    pub use std::sync::Arc;
}

pub mod business {
    pub use crate::structs::business::business_connection::BusinessConnection;
    pub use crate::structs::business::business_intro::BusinessIntro;
    pub use crate::structs::business::business_location::BusinessLocation;
    pub use crate::structs::business::business_message_deleted::BusinessMessagesDeleted;
    pub use crate::structs::business::business_opening_hours::BusinessOpeningHours;
    pub use crate::structs::business::business_opening_hours_interval::BusinessOpeningHoursInterval;
    pub use crate::traits::business::Business as BusinessTrait;
}

pub mod chat {
    pub use crate::structs::birthdate::Birthdate;
    pub use crate::structs::chat::Chat;
    pub use crate::structs::chats::chat_administrator_rights::ChatAdministratorRights;
    pub use crate::structs::chats::chat_background::ChatBackground;
    pub use crate::structs::chats::chat_boost::ChatBoost;
    pub use crate::structs::chats::chat_boost_added::ChatBoostAdded;
    pub use crate::structs::chats::chat_boost_removed::ChatBoostRemoved;
    pub use crate::structs::chats::chat_full_info::ChatFullInfo;
    pub use crate::structs::chats::chat_id::ChatId;
    pub use crate::structs::chats::chat_invite_link::ChatInviteLink;
    pub use crate::structs::chats::chat_join_request::ChatJoinRequest;
    pub use crate::structs::chats::chat_location::ChatLocation;
    pub use crate::structs::chats::chat_member_administrator::ChatMemberAdministrator;
    pub use crate::structs::chats::chat_member_banned::ChatMemberBanned;
    pub use crate::structs::chats::chat_member_left::ChatMemberLeft;
    pub use crate::structs::chats::chat_member_member::ChatMemberMember;
    pub use crate::structs::chats::chat_member_owner::ChatMemberOwner;
    pub use crate::structs::chats::chat_member_restricted::ChatMemberRestricted;
    pub use crate::structs::chats::chat_member_updated::ChatMemberUpdated;
    pub use crate::structs::chats::chat_permissions::ChatPermissions;
    pub use crate::structs::chats::chat_photo::ChatPhoto;
    pub use crate::structs::chats::chat_shared::ChatShared;
    pub use crate::structs::chats::options::Options as ChatOptions;
    pub use crate::structs::chats::user_chat_boosts::UserChatBoosts;
    pub use crate::structs::chats::video_chat_ended::VideoChatEnded;
    pub use crate::structs::chats::video_chat_participants_invited::VideoChatParticipantsInvited;
    pub use crate::structs::chats::video_chat_scheduled::VideoChatScheduled;
    pub use crate::structs::chats::video_chat_started::VideoChatStarted;
    pub use crate::traits::chat::Chat as ChatTrait;
}

pub mod chat_actions {
    pub use crate::enums::chat_action::ChatAction;
    pub use crate::traits::chat_actions::ChatActions as ChatActionsTrait;
}

pub mod contact {
    pub use crate::structs::contact::Contact;
    pub use crate::traits::contact::Contact as ContactTrait;
}

pub mod commands {
    pub use crate::traits::bots_apis::commands::Commands;
    pub use crate::traits::params::EnumParams as CommandsTrait;
    pub use telegram_macros::BotCommands;
}

pub mod dice {
    pub use crate::enums::emoji::Emoji;
    pub use crate::traits::dice::Dice as DiceTrait;
}

pub mod document {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::document::Document;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::messages::document::Document as DocumentMessage;
    pub use crate::traits::document::Document as DocumentTrait;
}

pub mod file {
    pub use crate::structs::file::File;
    pub use crate::traits::file::File as FileTrait;
}

pub mod forum {
    pub use crate::structs::forum_topics::forum_topic::ForumTopic;
    pub use crate::structs::forum_topics::forum_topic_closed::ForumTopicClosed;
    pub use crate::structs::forum_topics::forum_topic_created::ForumTopicEdited;
    pub use crate::structs::forum_topics::forum_topic_edited::ForumTopicCreated;
    pub use crate::structs::forum_topics::forum_topic_reopened::ForumTopicReopened;
    pub use crate::structs::forum_topics::general_forum_topic_hidden::GeneralForumTopicHidden;
    pub use crate::structs::forum_topics::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
    pub use crate::traits::forum::Forum as ForumTrait;
}

pub mod game {
    pub use crate::structs::game::Game;
    pub use crate::structs::games::callback_game::CallbackGame;
    pub use crate::structs::games::game_high_score::GameHighScore;
    pub use crate::structs::games::options as GameOptions;
    pub use crate::traits::game::Game as GameTrait;
}

pub mod http_listener {
    pub use crate::traits::bots_apis::http_listen::HttpListen as HttpListenTrait;
}

pub mod https_listener {
    pub use crate::traits::bots_apis::https_listen::HttpsListen as HttpsListenTrait;
}

pub mod inline {}

pub mod location {
    pub use crate::structs::location::Location;
    pub use crate::traits::location::Location as LocationTrait;
}

pub mod media_group {
    pub use crate::enums::input_media::InputMedia;
    pub use crate::structs::media::input_media_animation::InputMediaAnimation;
    pub use crate::structs::media::input_media_audio::InputMediaAudio;
    pub use crate::structs::media::input_media_document::InputMediaDocument;
    pub use crate::structs::media::input_media_photo::InputMediaPhoto;
    pub use crate::structs::media::input_media_video::InputMediaVideo;
    pub use crate::traits::media_group::MediaGroup as MediaGroupTrait;
}

pub mod menu_buttons {
    pub use crate::enums::menu_button::MenuButton;
    pub use crate::structs::menu_buttons::menu_button_commands::MenuButtonCommands;
    pub use crate::structs::menu_buttons::menu_button_default::MenuButtonDefault;
    pub use crate::structs::menu_buttons::menu_button_web_app::MenuButtonWebApp;
    pub use crate::traits::menu_button::MenuButton as MenuButtonTrait;
}

pub mod message {
    pub mod reactions {
        pub use crate::structs::reactions::message_reaction_count_updated::MessageReactionCountUpdated;
        pub use crate::structs::reactions::message_reaction_updated::MessageReactionUpdated;
        pub use crate::structs::reactions::reaction_count::ReactionCount;
        pub use crate::structs::reactions::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
        pub use crate::structs::reactions::reaction_type_emoji::ReactionTypeEmoji;
    }
}

pub mod my {
    pub use crate::structs::chats::chat_administrator_rights::ChatAdministratorRights;
    pub use crate::structs::my::bot_description::BotDescription;
    pub use crate::structs::my::bot_name::BotName;
    pub use crate::structs::my::bot_short_description::BotShortDescription;
    pub use crate::traits::my::My as MyTrait;
}

pub mod payments {
    pub mod invoice {
        pub use crate::structs::payments::invoice::Invoice;
        pub use crate::structs::payments::labeled_price::LabeledPrice;
        pub use crate::traits::payments::invoice::Invoice as InvoiceTrait;
    }

    pub mod order {
        pub use crate::structs::payments::labeled_price::LabeledPrice;
        pub use crate::structs::payments::shipping_address::ShippingAddress;
        pub use crate::structs::payments::shipping_option::ShippingOption;
        pub use crate::structs::payments::shipping_query::ShippingQuery;
        pub use crate::traits::payments::order::Order as OrderTrait;
    }

    pub mod star {
        pub use crate::enums::paid_media::PaidMedia;
        pub use crate::enums::revenue_withdrawal_state::RevenueWithdrawalState;
        pub use crate::enums::transaction_partner::TransactionPartner;
        pub use crate::structs::payments::paid_media_info::PaidMediaInfo;
        pub use crate::structs::payments::paid_media_photo::PaidMediaPhoto;
        pub use crate::structs::payments::paid_media_preview::PaidMediaPreview;
        pub use crate::structs::payments::paid_media_video::PaidMediaVideo;
        pub use crate::structs::payments::revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;
        pub use crate::structs::payments::revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;
        pub use crate::structs::payments::revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;
        pub use crate::structs::payments::transaction_partner_fragment::TransactionPartnerFragment;
        pub use crate::structs::payments::transaction_partner_other::TransactionPartnerOther;
        pub use crate::structs::payments::transaction_partner_telegram_ads::TransactionPartnerTelegramAds;
        pub use crate::structs::payments::transaction_partner_user::TransactionPartnerUser;
        pub use crate::traits::payments::star::Star as StarTrait;
    }

    pub use crate::structs::payments::options::Options;
}

pub mod passport {
    pub use crate::enums::passport_element_error::PassportElementError;
    pub use crate::structs::passports::encrypted_credentials::EncryptedCredentials;
    pub use crate::structs::passports::encrypted_passport_element::EncryptedPassportElement;
    pub use crate::structs::passports::passport_data::PassportData;
    pub use crate::structs::passports::passport_element_error_data_field::PassportElementErrorDataField;
    pub use crate::structs::passports::passport_element_error_file::PassportElementErrorFile;
    pub use crate::structs::passports::passport_element_error_files::PassportElementErrorFiles;
    pub use crate::structs::passports::passport_element_error_front_side::PassportElementErrorFrontSide;
    pub use crate::structs::passports::passport_element_error_reverse_side::PassportElementErrorReverseSide;
    pub use crate::structs::passports::passport_element_error_selfie::PassportElementErrorSelfie;
    pub use crate::structs::passports::passport_element_error_translation_file::PassportElementErrorTranslationFile;
    pub use crate::structs::passports::passport_element_error_translation_files::PassportElementErrorTranslationFiles;
    pub use crate::structs::passports::passport_element_error_unspecified::PassportElementErrorUnspecified;
    pub use crate::structs::passports::passport_file::PassportFile;
    pub use crate::traits::passport::Passport as PassportTrait;
}

pub mod photo {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::traits::photo::Photo as PhotoTrait;
}

pub mod pooling {
    pub use crate::traits::bots_apis::pooling::Pooling as PoolingTrait;
}

pub mod poll {
    pub use crate::structs::poll::Poll;
    pub use crate::structs::polls::input_poll_option::InputPollOption;
    pub use crate::structs::polls::options::Options as PollOptions;
    pub use crate::structs::polls::poll_answer::PollAnswer;
    pub use crate::structs::polls::poll_option::PollOption;
    pub use crate::traits::poll::Poll as PollTrait;
}

pub mod reply_markup {}

pub mod stickers {}

pub mod user {
    pub use crate::structs::user::User;
    pub use crate::structs::users::options::Options as UserOptions;
    pub use crate::structs::users::shared_user::SharedUser;
    pub use crate::structs::users::user_profile_photos::UserProfilePhotos;
    pub use crate::structs::users::users_shared::UsersShared;
    pub use crate::traits::user::User as UserTrait;
}

pub mod venue {
    pub use crate::structs::venue::Venue;
    pub use crate::traits::venue::Venue as VenueTrait;
}

pub mod video {
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::media::video::Video;
    pub use crate::structs::messages::voice::Voice as VideoMessage;
    pub use crate::traits::video::Video as VideoTrait;
}

pub mod video_note {
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::media::video_note::VideoNote;
    pub use crate::structs::messages::voice::Voice as VideoNoteMessage;
    pub use crate::traits::video_note::VideoNote as VideoNoteTrait;
}

pub mod voice {
    pub use crate::structs::media::voice::Voice;
    pub use crate::structs::messages::voice::Voice as VoiceMessage;
    pub use crate::traits::voice::Voice as VoiceTrait;
}

pub mod web_app {
    pub use crate::structs::web_apps::web_app_data::WebAppData;
    pub use crate::structs::web_apps::web_app_info::WebAppInfo;
}

pub mod webhook {
    pub use crate::traits::webhook::Webhook;
}
