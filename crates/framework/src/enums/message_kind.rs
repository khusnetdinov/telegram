use crate::structs::chats::chat_background::ChatBackground;
use crate::structs::chats::chat_shared::ChatShared;
use crate::structs::contact::Contact;
use crate::structs::dice::Dice;
use crate::structs::game::Game;
use crate::structs::geo::location::Location;
use crate::structs::geo::venue::Venue;
use crate::structs::giveaway::Giveaway;
use crate::structs::messages::message_kinds::animation_received::AnimationReceived;
use crate::structs::messages::message_kinds::audio_received::AudioReceived;
use crate::structs::messages::message_kinds::channel_chat_created::ChannelChatCreated;
use crate::structs::messages::message_kinds::chat_boost_added::ChatBoostAdded;
use crate::structs::messages::message_kinds::chat_member_left::ChatMemberLeft;
use crate::structs::messages::message_kinds::chat_photo_deleted::ChatPhotoDeleted;
use crate::structs::messages::message_kinds::command_received::CommandReceived;
use crate::structs::messages::message_kinds::document_received::DocumentReceived;
use crate::structs::messages::message_kinds::forum_topic_closed::ForumTopicClosed;
use crate::structs::messages::message_kinds::forum_topic_created::ForumTopicEdited;
use crate::structs::messages::message_kinds::forum_topic_edited::ForumTopicCreated;
use crate::structs::messages::message_kinds::forum_topic_reopened::ForumTopicReopened;
use crate::structs::messages::message_kinds::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::structs::messages::message_kinds::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::structs::messages::message_kinds::giveaway_completed::GiveawayCompleted;
use crate::structs::messages::message_kinds::giveaway_created::GiveawayCreated;
use crate::structs::messages::message_kinds::giveaway_winners::GiveawayWinners;
use crate::structs::messages::message_kinds::group_chat_created::GroupChatCreated;
use crate::structs::messages::message_kinds::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::structs::messages::message_kinds::message_pinned::MessagePinned;
use crate::structs::messages::message_kinds::migrate_from_chat::MigrateFromChat;
use crate::structs::messages::message_kinds::migrate_to_chat::MigrateToChat;
use crate::structs::messages::message_kinds::new_chat_members::NewChatMembersMessage;
use crate::structs::messages::message_kinds::new_chat_photo::NewChatPhotoMessage;
use crate::structs::messages::message_kinds::new_chat_title::NewChatTitle;
use crate::structs::messages::message_kinds::photo_sizes_received::PhotoSizesReceived;
use crate::structs::messages::message_kinds::proximity_alert_triggered::ProximityAlertTriggered;
use crate::structs::messages::message_kinds::sticker_received::StickerReceived;
use crate::structs::messages::message_kinds::story_received::StoryReceived;
use crate::structs::messages::message_kinds::supergroup_chat_created::SupergroupChatCreated;
use crate::structs::messages::message_kinds::text_received::TextReceived;
use crate::structs::messages::message_kinds::video_chat_ended::VideoChatEnded;
use crate::structs::messages::message_kinds::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::structs::messages::message_kinds::video_chat_scheduled::VideoChatScheduled;
use crate::structs::messages::message_kinds::video_chat_started::VideoChatStarted;
use crate::structs::messages::message_kinds::video_note_received::VideoNoteReceived;
use crate::structs::messages::message_kinds::video_received::VideoReceived;
use crate::structs::messages::message_kinds::voice_received::VoiceReceived;
use crate::structs::messages::message_kinds::website_connected::WebsiteConnected;
use crate::structs::messages::message_kinds::write_access_allowed::WriteAccessAllowed;
use crate::structs::passports::passport_data::PassportData;
use crate::structs::payments::invoice::Invoice;
use crate::structs::payments::successful_payment::SuccessfulPayment;
use crate::structs::poll::Poll;
use crate::structs::users::users_shared::UsersShared;
use crate::structs::web_apps::web_app_data::WebAppData;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageKind {
    /// Text messages, the actual UTF-8 text of the message
    Text(TextReceived),
    /// Command text messages, the actual UTF-8 text of the message
    Command(CommandReceived),
    /// Message is an animation, information about the animation. For backward compatibility,
    /// when this field is set, the document field will also be set
    Animation(AnimationReceived),
    /// Message is an audio file, information about the file
    Audio(AudioReceived),
    /// Message is a general file, information about the file
    Document(DocumentReceived),
    /// Message is a photo, available sizes of the photo
    Photo(PhotoSizesReceived),
    /// Message is a sticker, information about the sticker
    Sticker(StickerReceived),
    /// Message is a forwarded story
    Story(StoryReceived),
    /// Message is a video, information about the video
    Video(VideoReceived),
    /// Message is a video note, information about the video message
    VideoNote(VideoNoteReceived),
    /// Message is a voice message, information about the file
    Voice(VoiceReceived),
    /// Message is a shared contact, information about the contact
    Contact(Contact),
    /// Message is a dice with random value
    Dice(Dice),
    /// Message is a game, information about the game.
    Game(Game),
    /// Message is a native poll, information about the poll
    Poll(Poll),
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    Venue(Venue),
    ///  Message is a shared location, information about the location
    Location(Location),
    /// New members that were added to the group or supergroup and information about them (the bot
    /// itself may be one of these members)
    NewChatMembers(NewChatMembersMessage),
    /// A member was removed from the group, information about them (this member may be the bot itself)
    LeftChatMember(ChatMemberLeft),
    /// A chat title was changed to this value
    NewChatTitle(NewChatTitle),
    /// A chat photo was change to this value
    NewChatPhoto(NewChatPhotoMessage),
    /// Service message: the chat photo was deleted
    DeleteChatPhoto(ChatPhotoDeleted),
    /// Service message: the group has been created
    GroupChatCreated(GroupChatCreated),
    ///  Service message: the supergroup has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a
    /// directly created supergroup.
    SupergroupChatCreated(SupergroupChatCreated),
    /// Service message: the channel has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a channel when it is created. It can
    /// only be found in reply_to_message if someone replies to a very first message in a channel.
    ChannelChatCreated(ChannelChatCreated),
    /// Service message: auto-delete timer settings changed in the chat
    MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged),
    /// The group has been migrated to a supergroup with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have difficulty/silent
    /// defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer
    /// or double-precision float type are safe for storing this identifier.
    MigrateToChat(MigrateToChat),
    /// The supergroup has been migrated from a group with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have difficulty/silent
    /// defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer
    /// or double-precision float type are safe for storing this identifier.
    MigrateFromChat(MigrateFromChat),
    /// Specified message was pinned. Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it itself is a reply.
    Pinned(MessagePinned),
    /// Message is an invoice for a payment, information about the invoice.
    Invoice(Invoice),
    /// Message is a service message about a successful payment, information about the payment.
    SuccessfulPayment(SuccessfulPayment),
    /// Service message: users were shared with the bot
    UsersShared(UsersShared),
    /// Service message: a chat was shared with the bot
    ChatShared(ChatShared),
    /// The domain name of the website on which the user has logged in.
    ConnectedWebsite(WebsiteConnected),
    /// Service message: the user allowed the bot to write messages after adding it to the attachment
    /// or side menu, launching a Web App from a link, or accepting an explicit request from a Web App
    /// sent by the method requestWriteAccess
    WriteAccessAllowed(WriteAccessAllowed),
    /// Telegram Passport data
    PassportData(PassportData),
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    ProximityAlertTriggered(ProximityAlertTriggered),
    /// Service message: user boosted the chat
    ChatBoostAdded(ChatBoostAdded),
    /// Service message: chat background set
    ChatBackground(ChatBackground),
    ///  Service message: forum topic created
    ForumTopicCreated(ForumTopicCreated),
    /// Service message: forum topic edited
    ForumTopicEdited(ForumTopicEdited),
    /// Service message: forum topic closed
    ForumTopicClosed(ForumTopicClosed),
    /// Service message: forum topic reopened
    ForumTopicReopened(ForumTopicReopened),
    /// Service message: the 'General' forum topic hidden
    GeneralForumTopicHidden(GeneralForumTopicHidden),
    /// Service message: the 'General' forum topic unhidden
    GeneralForumTopicUnhidden(GeneralForumTopicUnhidden),
    /// Service message: a scheduled giveaway was created
    GiveawayCreated(GiveawayCreated),
    /// The message is a scheduled giveaway message
    Giveaway(Giveaway),
    /// A giveaway with public winners was completed
    GiveawayWinners(GiveawayWinners),
    /// Service message: a giveaway without public winners was completed
    GiveawayCompleted(GiveawayCompleted),
    /// Service message: video chat scheduled
    VideoChatScheduled(VideoChatScheduled),
    /// Service message: video chat started
    VideoChatStarted(VideoChatStarted),
    /// Service message: video chat ended
    VideoChatEnded(VideoChatEnded),
    /// Service message: new participants invited to a video chat
    VideoChatParticipantsInvited(VideoChatParticipantsInvited),
    /// Service message: data sent by a Web App
    WebAppData(WebAppData),
    /// Not Telegram type: for unexpected messages, errors, debugging, logging purpose.
    Unexpected(Inner),
}

impl From<Inner> for MessageKind {
    fn from(inner: Inner) -> Self {
        match inner {
            inner if Self::is_text(&inner) => MessageKind::Text(TextReceived::from(inner)),
            inner if Self::is_command(&inner) => MessageKind::Command(CommandReceived::from(inner)),
            inner if Self::is_animation(&inner) => {
                MessageKind::Animation(AnimationReceived::from(inner))
            }
            inner if Self::is_audio(&inner) => MessageKind::Audio(AudioReceived::from(inner)),
            inner if Self::is_document(&inner) => {
                MessageKind::Document(DocumentReceived::from(inner))
            }
            inner if Self::is_photo(&inner) => MessageKind::Photo(PhotoSizesReceived::from(inner)),
            inner if Self::is_sticker(&inner) => MessageKind::Sticker(StickerReceived::from(inner)),
            inner if Self::is_contact(&inner) => MessageKind::Contact(Contact::from(inner)),
            inner if Self::is_dice(&inner) => MessageKind::Dice(Dice::from(inner)),
            inner if Self::is_game(&inner) => MessageKind::Game(Game::from(inner)),
            inner if Self::is_poll(&inner) => MessageKind::Poll(Poll::from(inner)),
            inner if Self::is_venue(&inner) => MessageKind::Venue(Venue::from(inner)),
            inner if Self::is_location(&inner) => MessageKind::Location(Location::from(inner)),
            inner if Self::is_new_chat_members(&inner) => {
                MessageKind::NewChatMembers(NewChatMembersMessage::from(inner))
            }
            inner if Self::is_left_chat_member(&inner) => {
                MessageKind::LeftChatMember(ChatMemberLeft::from(inner))
            }
            inner if Self::is_new_chat_title(&inner) => {
                MessageKind::NewChatTitle(NewChatTitle::from(inner))
            }
            inner if Self::is_new_chat_photo(&inner) => {
                MessageKind::NewChatPhoto(NewChatPhotoMessage::from(inner))
            }
            inner if Self::is_delete_chat_photo(&inner) => {
                MessageKind::DeleteChatPhoto(ChatPhotoDeleted::from(inner))
            }
            inner if Self::is_group_chat_created(&inner) => {
                MessageKind::GroupChatCreated(GroupChatCreated::from(inner))
            }
            inner if Self::is_supergroup_chat_created(&inner) => {
                MessageKind::SupergroupChatCreated(SupergroupChatCreated::from(inner))
            }
            inner if Self::is_channel_chat_created(&inner) => {
                MessageKind::ChannelChatCreated(ChannelChatCreated::from(inner))
            }
            inner if Self::is_message_auto_delete_timer_changed(&inner) => {
                MessageKind::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged::from(
                    inner,
                ))
            }
            inner if Self::is_migrate_to_chat_id(&inner) => {
                MessageKind::MigrateToChat(MigrateToChat::from(inner))
            }
            inner if Self::is_migrate_from_chat_id(&inner) => {
                MessageKind::MigrateFromChat(MigrateFromChat::from(inner))
            }
            inner if Self::is_pinned_message(&inner) => {
                MessageKind::Pinned(MessagePinned::from(inner))
            }
            inner if Self::is_invoice(&inner) => MessageKind::Invoice(Invoice::from(inner)),
            inner if Self::is_successful_payment(&inner) => {
                MessageKind::SuccessfulPayment(SuccessfulPayment::from(inner))
            }
            inner if Self::is_users_shared(&inner) => {
                MessageKind::UsersShared(UsersShared::from(inner))
            }
            inner if Self::is_chat_shared(&inner) => {
                MessageKind::ChatShared(ChatShared::from(inner))
            }
            inner if Self::is_connected_website(&inner) => {
                MessageKind::ConnectedWebsite(WebsiteConnected::from(inner))
            }
            inner if Self::is_write_access_allowed(&inner) => {
                MessageKind::WriteAccessAllowed(WriteAccessAllowed::from(inner))
            }
            inner if Self::is_passport_data(&inner) => {
                MessageKind::PassportData(PassportData::from(inner))
            }
            inner if Self::is_proximity_alert_triggered(&inner) => {
                MessageKind::ProximityAlertTriggered(ProximityAlertTriggered::from(inner))
            }
            inner if Self::is_boost_added(&inner) => {
                MessageKind::ChatBoostAdded(ChatBoostAdded::from(inner))
            }
            inner if Self::is_chat_background_set(&inner) => {
                MessageKind::ChatBackground(ChatBackground::from(inner))
            }
            inner if Self::is_forum_topic_edited(&inner) => {
                MessageKind::ForumTopicEdited(ForumTopicEdited::from(inner))
            }
            inner if Self::is_forum_topic_created(&inner) => {
                MessageKind::ForumTopicCreated(ForumTopicCreated::from(inner))
            }
            inner if Self::is_forum_topic_closed(&inner) => {
                MessageKind::ForumTopicClosed(ForumTopicClosed::from(inner))
            }
            inner if Self::is_forum_topic_reopened(&inner) => {
                MessageKind::ForumTopicReopened(ForumTopicReopened::from(inner))
            }
            inner if Self::is_general_forum_topic_hidden(&inner) => {
                MessageKind::GeneralForumTopicHidden(GeneralForumTopicHidden::from(inner))
            }
            inner if Self::is_general_forum_topic_unhidden(&inner) => {
                MessageKind::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden::from(inner))
            }
            // inner if Self::is_giveaway_created(&inner) => {
            //     Messages::GiveawayCreated(GiveawayCreated::from(inner))
            // }
            inner if Self::is_giveaway(&inner) => MessageKind::Giveaway(Giveaway::from(inner)),
            // inner if Self::is_giveaway_winners(&inner) => {
            //     Messages::GiveawayWinners(GiveawayWinners::from(inner))
            // }
            inner if Self::is_giveaway_completed(&inner) => {
                MessageKind::GiveawayCompleted(GiveawayCompleted::from(inner))
            }
            inner if Self::is_video_chat_scheduled(&inner) => {
                MessageKind::VideoChatScheduled(VideoChatScheduled::from(inner))
            }
            inner if Self::is_video_chat_started(&inner) => {
                MessageKind::VideoChatStarted(VideoChatStarted::from(inner))
            }
            inner if Self::is_video_chat_ended(&inner) => {
                MessageKind::VideoChatEnded(VideoChatEnded::from(inner))
            }
            inner if Self::is_video_chat_participants_invited(&inner) => {
                MessageKind::VideoChatParticipantsInvited(VideoChatParticipantsInvited::from(inner))
            }
            inner if Self::is_web_app_data(&inner) => {
                MessageKind::WebAppData(WebAppData::from(inner))
            }
            _ => Self::Unexpected(inner),
        }
    }
}

impl MessageKind {
    pub fn is_text(inner: &Inner) -> bool {
        let bot_command_entity_none = if inner.entities.is_some() {
            !inner
                .entities
                .as_ref()
                .unwrap()
                .iter()
                .any(|entity: &MessageEntity| entity.kind.as_str() == "bot_command")
        } else {
            true
        };

        inner.text.is_some() && bot_command_entity_none
    }

    pub fn is_command(inner: &Inner) -> bool {
        let bot_command_entity_any = inner.entities.is_some()
            && inner
                .entities
                .as_ref()
                .unwrap()
                .iter()
                .any(|entity: &MessageEntity| entity.kind.as_str() == "bot_command");

        inner.text.is_some() && bot_command_entity_any
    }

    pub fn is_animation(inner: &Inner) -> bool {
        inner.animation.is_some()
    }

    pub fn is_audio(inner: &Inner) -> bool {
        inner.audio.is_some()
    }

    pub fn is_document(inner: &Inner) -> bool {
        inner.document.is_some() && inner.animation.is_none()
    }

    pub fn is_photo(inner: &Inner) -> bool {
        inner.photo.is_some()
    }

    pub fn is_sticker(inner: &Inner) -> bool {
        inner.sticker.is_some()
    }

    pub fn is_contact(inner: &Inner) -> bool {
        inner.contact.is_some()
    }

    pub fn is_dice(inner: &Inner) -> bool {
        inner.dice.is_some()
    }

    pub fn is_game(inner: &Inner) -> bool {
        inner.game.is_some()
    }

    pub fn is_poll(inner: &Inner) -> bool {
        inner.poll.is_some()
    }

    pub fn is_venue(inner: &Inner) -> bool {
        inner.venue.is_some()
    }

    pub fn is_location(inner: &Inner) -> bool {
        inner.location.is_some()
    }

    pub fn is_new_chat_members(inner: &Inner) -> bool {
        inner.new_chat_members.is_some()
    }

    pub fn is_left_chat_member(inner: &Inner) -> bool {
        inner.left_chat_member.is_some()
    }

    pub fn is_new_chat_title(inner: &Inner) -> bool {
        inner.new_chat_title.is_some()
    }

    pub fn is_new_chat_photo(inner: &Inner) -> bool {
        inner.new_chat_photo.is_some()
    }

    pub fn is_delete_chat_photo(inner: &Inner) -> bool {
        inner.delete_chat_photo.is_some()
    }

    pub fn is_group_chat_created(inner: &Inner) -> bool {
        inner.group_chat_created.is_some()
    }

    pub fn is_supergroup_chat_created(inner: &Inner) -> bool {
        inner.supergroup_chat_created.is_some()
    }

    pub fn is_channel_chat_created(inner: &Inner) -> bool {
        inner.channel_chat_created.is_some()
    }

    pub fn is_message_auto_delete_timer_changed(inner: &Inner) -> bool {
        inner.message_auto_delete_timer_changed.is_some()
    }

    pub fn is_migrate_to_chat_id(inner: &Inner) -> bool {
        inner.migrate_to_chat_id.is_some()
    }

    pub fn is_migrate_from_chat_id(inner: &Inner) -> bool {
        inner.migrate_from_chat_id.is_some()
    }

    pub fn is_pinned_message(inner: &Inner) -> bool {
        inner.pinned_message.is_some()
    }

    pub fn is_invoice(inner: &Inner) -> bool {
        inner.invoice.is_some()
    }

    pub fn is_successful_payment(inner: &Inner) -> bool {
        inner.successful_payment.is_some()
    }

    pub fn is_users_shared(inner: &Inner) -> bool {
        inner.users_shared.is_some()
    }

    pub fn is_chat_shared(inner: &Inner) -> bool {
        inner.chat_shared.is_some()
    }

    pub fn is_connected_website(inner: &Inner) -> bool {
        inner.connected_website.is_some()
    }

    pub fn is_write_access_allowed(inner: &Inner) -> bool {
        inner.write_access_allowed.is_some()
    }

    pub fn is_passport_data(inner: &Inner) -> bool {
        inner.passport_data.is_some()
    }

    pub fn is_proximity_alert_triggered(inner: &Inner) -> bool {
        inner.proximity_alert_triggered.is_some()
    }

    pub fn is_boost_added(inner: &Inner) -> bool {
        inner.boost_added.is_some()
    }

    pub fn is_chat_background_set(inner: &Inner) -> bool {
        inner.chat_background_set.is_some()
    }

    pub fn is_forum_topic_edited(inner: &Inner) -> bool {
        inner.forum_topic_edited.is_some()
    }

    pub fn is_forum_topic_created(inner: &Inner) -> bool {
        inner.forum_topic_created.is_some()
    }

    pub fn is_forum_topic_closed(inner: &Inner) -> bool {
        inner.forum_topic_closed.is_some()
    }

    pub fn is_forum_topic_reopened(inner: &Inner) -> bool {
        inner.forum_topic_reopened.is_some()
    }

    pub fn is_general_forum_topic_hidden(inner: &Inner) -> bool {
        inner.general_forum_topic_hidden.is_some()
    }

    pub fn is_general_forum_topic_unhidden(inner: &Inner) -> bool {
        inner.general_forum_topic_unhidden.is_some()
    }

    pub fn is_giveaway_created(inner: &Inner) -> bool {
        inner.giveaway_created.is_some()
    }

    pub fn is_giveaway(inner: &Inner) -> bool {
        inner.giveaway.is_some()
    }

    pub fn is_giveaway_winners(inner: &Inner) -> bool {
        inner.giveaway_winners.is_some()
    }

    pub fn is_giveaway_completed(inner: &Inner) -> bool {
        inner.giveaway_completed.is_some()
    }

    pub fn is_video_chat_scheduled(inner: &Inner) -> bool {
        inner.video_chat_scheduled.is_some()
    }

    pub fn is_video_chat_started(inner: &Inner) -> bool {
        inner.video_chat_started.is_some()
    }

    pub fn is_video_chat_ended(inner: &Inner) -> bool {
        inner.video_chat_ended.is_some()
    }

    pub fn is_video_chat_participants_invited(inner: &Inner) -> bool {
        inner.video_chat_participants_invited.is_some()
    }

    pub fn is_web_app_data(inner: &Inner) -> bool {
        inner.web_app_data.is_some()
    }
}
