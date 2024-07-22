pub mod animation {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::animation::Animation;
    pub use crate::structs::media::document::Document;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::structs::media::photo_size::PhotoSize;
    pub use crate::structs::messages::animation::Animation as AnimationMessage;
    pub use crate::traits::animation::Animation as AnimationTrait;
    pub use std::path::PathBuf;
}

pub mod audio {}

pub mod bots_api {
    pub use crate::bots_api::BotsApi;
    pub use crate::enums::messages::Messages;
    pub use crate::enums::updates::Updates;
    pub use crate::storages::memory::MemoryStorage;
    pub use crate::structs::options::Options;
    pub use crate::structs::update::Update;
    pub use crate::traits::kind_dispatcher::KindDispatcher;
    pub use std::fmt::Debug;
    pub use std::sync::Arc;
}

pub mod business {}

pub mod callback_query {}

pub mod chat {}

pub mod chat_actions {
    pub use crate::enums::chat_action::ChatAction;
    pub use crate::traits::chat_actions::ChatActions;
}

pub mod contact {
    pub use crate::structs::contact::Contact;
    pub use crate::traits::contact::Contact as ContactTrait;
}

pub mod commands {
    pub use crate::traits::bots_apis::commands::Commands;
    pub use crate::traits::params::EnumParams;
    pub use telegram_macros::BotCommands;
}

pub mod dice {
    pub use crate::enums::emoji::Emoji;
    pub use crate::traits::dice::Dice;
}

pub mod document {}

pub mod file {}

pub mod forum {}

pub mod game {
    pub use crate::structs::game::Game;
    pub use crate::traits::game::Game as GameTrait;
}

pub mod http_listener {
    pub use crate::traits::bots_apis::http_listen::HttpListen;
}

pub mod https_listener {
    pub use crate::traits::bots_apis::https_listen::HttpsListen;
}

pub mod inline {}

pub mod invoice {}

pub mod location {
    pub use crate::structs::location::Location;
    pub use crate::traits::location::Location as LocationTrait;
}

pub mod media_group {}

pub mod menu_buttons {}

pub mod message {}

pub mod my {}

pub mod order {}

pub mod passport {}

pub mod photo {
    pub use crate::enums::file_input::FileInput;
    pub use crate::structs::media::options::Options as MediaOptions;
    pub use crate::traits::photo::Photo as PhotoTrait;
    pub use std::path::PathBuf;
}

pub mod pooling {
    pub use crate::traits::bots_apis::pooling::Pooling;
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

pub mod star {}

pub mod stickers {}

pub mod user {}

pub mod venue {
    pub use crate::structs::venue::Venue;
    pub use crate::traits::venue::Venue as VenueTrait;
}

pub mod video {
    pub use crate::structs::media::video::Video;
    pub use crate::structs::messages::voice::Voice as VideoMessage;
    pub use crate::traits::video::Video as VideoTrait;
    pub use std::path::PathBuf;
}

pub mod video_note {
    pub use crate::structs::media::video_note::VideoNote;
    pub use crate::structs::messages::voice::Voice as VideoNoteMessage;
    pub use crate::traits::video_note::VideoNote as VideoNoteTrait;
    pub use std::path::PathBuf;
}

pub mod voice {
    pub use crate::structs::media::voice::Voice;
    pub use crate::structs::messages::voice::Voice as VoiceMessage;
    pub use crate::traits::voice::Voice as VoiceTrait;
    pub use std::path::PathBuf;
}

pub mod web_app {}

pub mod webhook {
    pub use crate::traits::webhook::Webhook;
}
