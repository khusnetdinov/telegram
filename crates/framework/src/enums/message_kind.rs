use crate::structs::message_kinds::animation_message::AnimationMessage;
use crate::structs::message_kinds::audio_message::AudioMessage;
use crate::structs::message_kinds::boost_added::ChatBoostAdded;
use crate::structs::message_kinds::channel_chat_created::ChannelChatCreated;
use crate::structs::message_kinds::chat_background::ChatBackground;
use crate::structs::message_kinds::chat_shared::ChatShared;
use crate::structs::message_kinds::command_message::CommandMessage;
use crate::structs::message_kinds::connected_website_message::ConnectedWebsiteMessage;
use crate::structs::message_kinds::contract_message::ContactMessage;
use crate::structs::message_kinds::delete_chat_photo_message::DeleteChatPhotoMessage;
use crate::structs::message_kinds::dice::Dice;
use crate::structs::message_kinds::document_message::DocumentMessage;
use crate::structs::message_kinds::forum_topic_closed_message::ForumTopicClosedMessage;
use crate::structs::message_kinds::forum_topic_created_message::ForumTopicEditedMessage;
use crate::structs::message_kinds::forum_topic_edited_message::ForumTopicCreatedMessage;
use crate::structs::message_kinds::forum_topic_reopened_message::ForumTopicReopenedMessage;
use crate::structs::message_kinds::game_message::GameMessage;
use crate::structs::message_kinds::general_forum_topic_hidden_message::GeneralForumTopicHiddenMessage;
use crate::structs::message_kinds::general_forum_topic_unhidden_message::GeneralForumTopicUnhiddenMessage;
use crate::structs::message_kinds::giveaway_completed_message::GiveawayCompletedMessage;
use crate::structs::message_kinds::giveaway_created_message::GiveawayCreatedMessage;
use crate::structs::message_kinds::giveaway_message::GiveawayMessage;
use crate::structs::message_kinds::giveaway_winners_message::GiveawayWinnersMessage;
use crate::structs::message_kinds::group_chat_created_message::GroupChatCreatedMessage;
use crate::structs::message_kinds::invoice_message::InvoiceMessage;
use crate::structs::message_kinds::left_chat_membe_message::LeftChatMemberMessage;
use crate::structs::message_kinds::location::Location;
use crate::structs::message_kinds::message_auto_delete_timer_changed_message::MessageAutoDeleteTimerChangedMessage;
use crate::structs::message_kinds::migrate_from_chat_message::MigrateFromChatMessage;
use crate::structs::message_kinds::migrate_to_chat_message::MigrateToChatMessage;
use crate::structs::message_kinds::new_chat_members_message::NewChatMembersMessage;
use crate::structs::message_kinds::new_chat_photo_message::NewChatPhotoMessage;
use crate::structs::message_kinds::new_chat_title_message::NewChatTitleMessage;
use crate::structs::message_kinds::passport_data_message::PassportDataMessage;
use crate::structs::message_kinds::photo_message::PhotoMessage;
use crate::structs::message_kinds::pinned_message::PinnedMessage;
use crate::structs::message_kinds::poll::Poll;
use crate::structs::message_kinds::proximity_alert_triggered_message::ProximityAlertTriggeredMessage;
use crate::structs::message_kinds::sticker_message::StickerMessage;
use crate::structs::message_kinds::story_message::StoryMessage;
use crate::structs::message_kinds::successful_payment_message::SuccessfulPaymentMessage;
use crate::structs::message_kinds::supergroup_chat_created_message::SupergroupChatCreatedMessage;
use crate::structs::message_kinds::text_message::TextMessage;
use crate::structs::message_kinds::users_shared_message::UsersSharedMessage;
use crate::structs::message_kinds::venue_message::VenueMessage;
use crate::structs::message_kinds::video_chat_ended_message::VideoChatEndedMessage;
use crate::structs::message_kinds::video_chat_participants_invited_message::VideoChatParticipantsInvitedMessage;
use crate::structs::message_kinds::video_chat_scheduled_message::VideoChatScheduledMessage;
use crate::structs::message_kinds::video_chat_started_message::VideoChatStartedMessage;
use crate::structs::message_kinds::video_message::VideoMessage;
use crate::structs::message_kinds::video_note_message::VideoNoteMessage;
use crate::structs::message_kinds::voice_message::VoiceMessage;
use crate::structs::message_kinds::web_app_data_message::WebAppDataMessage;
use crate::structs::message_kinds::write_access_allowed_message::WriteAccessAllowedMessage;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageKind {
    /// Text messages, the actual UTF-8 text of the message
    Text(TextMessage),
    /// Command text messages, the actual UTF-8 text of the message
    Command(CommandMessage),
    /// Message is an animation, information about the animation. For backward compatibility,
    /// when this field is set, the document field will also be set
    Animation(AnimationMessage),
    /// Message is an audio file, information about the file
    Audio(AudioMessage),
    /// Message is a general file, information about the file
    Document(DocumentMessage),
    /// Message is a photo, available sizes of the photo
    Photo(PhotoMessage),
    /// Message is a sticker, information about the sticker
    Sticker(StickerMessage),
    /// Message is a forwarded story
    Story(StoryMessage),
    /// Message is a video, information about the video
    Video(VideoMessage),
    /// Message is a video note, information about the video message
    VideoNote(VideoNoteMessage),
    /// Message is a voice message, information about the file
    Voice(VoiceMessage),
    /// Message is a shared contact, information about the contact
    Contact(ContactMessage),
    /// Message is a dice with random value
    Dice(Dice),
    /// Message is a game, information about the game.
    Game(GameMessage),
    /// Message is a native poll, information about the poll
    Poll(Poll),
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    Venue(VenueMessage),
    ///  Message is a shared location, information about the location
    Location(Location),
    /// New members that were added to the group or supergroup and information about them (the bot
    /// itself may be one of these members)
    NewChatMembers(NewChatMembersMessage),
    /// A member was removed from the group, information about them (this member may be the bot itself)
    LeftChatMember(LeftChatMemberMessage),
    /// A chat title was changed to this value
    NewChatTitle(NewChatTitleMessage),
    /// A chat photo was change to this value
    NewChatPhoto(NewChatPhotoMessage),
    /// Service message: the chat photo was deleted
    DeleteChatPhoto(DeleteChatPhotoMessage),
    /// Service message: the group has been created
    GroupChatCreated(GroupChatCreatedMessage),
    ///  Service message: the supergroup has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a supergroup when it is created.
    /// It can only be found in reply_to_message if someone replies to a very first message in a
    /// directly created supergroup.
    SupergroupChatCreated(SupergroupChatCreatedMessage),
    /// Service message: the channel has been created. This field can't be received in a message
    /// coming through updates, because bot can't be a member of a channel when it is created. It can
    /// only be found in reply_to_message if someone replies to a very first message in a channel.
    ChannelChatCreated(ChannelChatCreated),
    /// Service message: auto-delete timer settings changed in the chat
    MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChangedMessage),
    /// The group has been migrated to a supergroup with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have difficulty/silent
    /// defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer
    /// or double-precision float type are safe for storing this identifier.
    MigrateToChat(MigrateToChatMessage),
    /// The supergroup has been migrated from a group with the specified identifier. This number may
    /// have more than 32 significant bits and some programming languages may have difficulty/silent
    /// defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer
    /// or double-precision float type are safe for storing this identifier.
    MigrateFromChat(MigrateFromChatMessage),
    /// Specified message was pinned. Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it itself is a reply.
    Pinned(PinnedMessage),
    /// Message is an invoice for a payment, information about the invoice.
    Invoice(InvoiceMessage),
    /// Message is a service message about a successful payment, information about the payment.
    SuccessfulPayment(SuccessfulPaymentMessage),
    /// Service message: users were shared with the bot
    UsersShared(UsersSharedMessage),
    /// Service message: a chat was shared with the bot
    ChatShared(ChatShared),
    /// The domain name of the website on which the user has logged in.
    ConnectedWebsite(ConnectedWebsiteMessage),
    /// Service message: the user allowed the bot to write messages after adding it to the attachment
    /// or side menu, launching a Web App from a link, or accepting an explicit request from a Web App
    /// sent by the method requestWriteAccess
    WriteAccessAllowed(WriteAccessAllowedMessage),
    /// Telegram Passport data
    PassportData(PassportDataMessage),
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    ProximityAlertTriggered(ProximityAlertTriggeredMessage),
    /// Service message: user boosted the chat
    ChatBoostAdded(ChatBoostAdded),
    /// Service message: chat background set
    ChatBackground(ChatBackground),
    ///  Service message: forum topic created
    ForumTopicCreated(ForumTopicCreatedMessage),
    /// Service message: forum topic edited
    ForumTopicEdited(ForumTopicEditedMessage),
    /// Service message: forum topic closed
    ForumTopicClosed(ForumTopicClosedMessage),
    /// Service message: forum topic reopened
    ForumTopicReopened(ForumTopicReopenedMessage),
    /// Service message: the 'General' forum topic hidden
    GeneralForumTopicHidden(GeneralForumTopicHiddenMessage),
    /// Service message: the 'General' forum topic unhidden
    GeneralForumTopicUnhidden(GeneralForumTopicUnhiddenMessage),
    /// Service message: a scheduled giveaway was created
    GiveawayCreated(GiveawayCreatedMessage),
    /// The message is a scheduled giveaway message
    Giveaway(GiveawayMessage),
    /// A giveaway with public winners was completed
    GiveawayWinners(GiveawayWinnersMessage),
    /// Service message: a giveaway without public winners was completed
    GiveawayCompleted(GiveawayCompletedMessage),
    /// Service message: video chat scheduled
    VideoChatScheduled(VideoChatScheduledMessage),
    /// Service message: video chat started
    VideoChatStarted(VideoChatStartedMessage),
    /// Service message: video chat ended
    VideoChatEnded(VideoChatEndedMessage),
    /// Service message: new participants invited to a video chat
    VideoChatParticipantsInvited(VideoChatParticipantsInvitedMessage),
    /// Service message: data sent by a Web App
    WebAppData(WebAppDataMessage),
    /// Not Telegram type: for unexpected messages, errors, debugging, logging purpose.
    Unexpected(Inner),
}

impl From<Inner> for MessageKind {
    fn from(inner: Inner) -> Self {
        match inner {
            inner if Self::is_text(&inner) => MessageKind::Text(TextMessage::from(inner)),
            inner if Self::is_command(&inner) => MessageKind::Command(CommandMessage::from(inner)),
            inner if Self::is_animation(&inner) => {
                MessageKind::Animation(AnimationMessage::from(inner))
            }
            inner if Self::is_audio(&inner) => MessageKind::Audio(AudioMessage::from(inner)),
            inner if Self::is_document(&inner) => {
                MessageKind::Document(DocumentMessage::from(inner))
            }
            inner if Self::is_photo(&inner) => MessageKind::Photo(PhotoMessage::from(inner)),
            inner if Self::is_sticker(&inner) => MessageKind::Sticker(StickerMessage::from(inner)),
            inner if Self::is_contact(&inner) => MessageKind::Contact(ContactMessage::from(inner)),
            inner if Self::is_dice(&inner) => MessageKind::Dice(Dice::from(inner)),
            inner if Self::is_game(&inner) => MessageKind::Game(GameMessage::from(inner)),
            inner if Self::is_poll(&inner) => MessageKind::Poll(Poll::from(inner)),
            inner if Self::is_venue(&inner) => MessageKind::Venue(VenueMessage::from(inner)),
            inner if Self::is_location(&inner) => MessageKind::Location(Location::from(inner)),
            inner if Self::is_new_chat_members(&inner) => {
                MessageKind::NewChatMembers(NewChatMembersMessage::from(inner))
            }
            inner if Self::is_left_chat_member(&inner) => {
                MessageKind::LeftChatMember(LeftChatMemberMessage::from(inner))
            }
            inner if Self::is_new_chat_title(&inner) => {
                MessageKind::NewChatTitle(NewChatTitleMessage::from(inner))
            }
            inner if Self::is_new_chat_photo(&inner) => {
                MessageKind::NewChatPhoto(NewChatPhotoMessage::from(inner))
            }
            inner if Self::is_delete_chat_photo(&inner) => {
                MessageKind::DeleteChatPhoto(DeleteChatPhotoMessage::from(inner))
            }
            inner if Self::is_group_chat_created(&inner) => {
                MessageKind::GroupChatCreated(GroupChatCreatedMessage::from(inner))
            }
            inner if Self::is_supergroup_chat_created(&inner) => {
                MessageKind::SupergroupChatCreated(SupergroupChatCreatedMessage::from(inner))
            }
            inner if Self::is_channel_chat_created(&inner) => {
                MessageKind::ChannelChatCreated(ChannelChatCreated::from(inner))
            }
            inner if Self::is_message_auto_delete_timer_changed(&inner) => {
                MessageKind::MessageAutoDeleteTimerChanged(
                    MessageAutoDeleteTimerChangedMessage::from(inner),
                )
            }
            inner if Self::is_migrate_to_chat_id(&inner) => {
                MessageKind::MigrateToChat(MigrateToChatMessage::from(inner))
            }
            inner if Self::is_migrate_from_chat_id(&inner) => {
                MessageKind::MigrateFromChat(MigrateFromChatMessage::from(inner))
            }
            inner if Self::is_pinned_message(&inner) => {
                MessageKind::Pinned(PinnedMessage::from(inner))
            }
            inner if Self::is_invoice(&inner) => MessageKind::Invoice(InvoiceMessage::from(inner)),
            inner if Self::is_successful_payment(&inner) => {
                MessageKind::SuccessfulPayment(SuccessfulPaymentMessage::from(inner))
            }
            inner if Self::is_users_shared(&inner) => {
                MessageKind::UsersShared(UsersSharedMessage::from(inner))
            }
            inner if Self::is_chat_shared(&inner) => {
                MessageKind::ChatShared(ChatShared::from(inner))
            }
            inner if Self::is_connected_website(&inner) => {
                MessageKind::ConnectedWebsite(ConnectedWebsiteMessage::from(inner))
            }
            inner if Self::is_write_access_allowed(&inner) => {
                MessageKind::WriteAccessAllowed(WriteAccessAllowedMessage::from(inner))
            }
            inner if Self::is_passport_data(&inner) => {
                MessageKind::PassportData(PassportDataMessage::from(inner))
            }
            inner if Self::is_proximity_alert_triggered(&inner) => {
                MessageKind::ProximityAlertTriggered(ProximityAlertTriggeredMessage::from(inner))
            }
            inner if Self::is_boost_added(&inner) => {
                MessageKind::ChatBoostAdded(ChatBoostAdded::from(inner))
            }
            inner if Self::is_chat_background_set(&inner) => {
                MessageKind::ChatBackground(ChatBackground::from(inner))
            }
            inner if Self::is_forum_topic_edited(&inner) => {
                MessageKind::ForumTopicEdited(ForumTopicEditedMessage::from(inner))
            }
            inner if Self::is_forum_topic_created(&inner) => {
                MessageKind::ForumTopicCreated(ForumTopicCreatedMessage::from(inner))
            }
            inner if Self::is_forum_topic_closed(&inner) => {
                MessageKind::ForumTopicClosed(ForumTopicClosedMessage::from(inner))
            }
            inner if Self::is_forum_topic_reopened(&inner) => {
                MessageKind::ForumTopicReopened(ForumTopicReopenedMessage::from(inner))
            }
            inner if Self::is_general_forum_topic_hidden(&inner) => {
                MessageKind::GeneralForumTopicHidden(GeneralForumTopicHiddenMessage::from(inner))
            }
            inner if Self::is_general_forum_topic_unhidden(&inner) => {
                MessageKind::GeneralForumTopicUnhidden(GeneralForumTopicUnhiddenMessage::from(
                    inner,
                ))
            }
            inner if Self::is_giveaway_created(&inner) => {
                MessageKind::GiveawayCreated(GiveawayCreatedMessage::from(inner))
            }
            inner if Self::is_giveaway(&inner) => {
                MessageKind::Giveaway(GiveawayMessage::from(inner))
            }
            inner if Self::is_giveaway_winners(&inner) => {
                MessageKind::GiveawayWinners(GiveawayWinnersMessage::from(inner))
            }
            inner if Self::is_giveaway_completed(&inner) => {
                MessageKind::GiveawayCompleted(GiveawayCompletedMessage::from(inner))
            }
            inner if Self::is_video_chat_scheduled(&inner) => {
                MessageKind::VideoChatScheduled(VideoChatScheduledMessage::from(inner))
            }
            inner if Self::is_video_chat_started(&inner) => {
                MessageKind::VideoChatStarted(VideoChatStartedMessage::from(inner))
            }
            inner if Self::is_video_chat_ended(&inner) => {
                MessageKind::VideoChatEnded(VideoChatEndedMessage::from(inner))
            }
            inner if Self::is_video_chat_participants_invited(&inner) => {
                MessageKind::VideoChatParticipantsInvited(
                    VideoChatParticipantsInvitedMessage::from(inner),
                )
            }
            inner if Self::is_web_app_data(&inner) => {
                MessageKind::WebAppData(WebAppDataMessage::from(inner))
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
