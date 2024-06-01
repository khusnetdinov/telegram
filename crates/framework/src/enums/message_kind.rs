use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use telegram_bots_api::api::structs::animation::Animation;
use telegram_bots_api::api::structs::audio::Audio;
use telegram_bots_api::api::structs::chat_background::ChatBackground;
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded;
use telegram_bots_api::api::structs::chat_shared::ChatShared;
use telegram_bots_api::api::structs::contact::Contact;
use telegram_bots_api::api::structs::dice::Dice;
use telegram_bots_api::api::structs::document::Document;
use telegram_bots_api::api::structs::forum_topic_closed::ForumTopicClosed;
use telegram_bots_api::api::structs::forum_topic_created::ForumTopicCreated;
use telegram_bots_api::api::structs::forum_topic_edited::ForumTopicEdited;
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened;
use telegram_bots_api::api::structs::game::Game;
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden;
use telegram_bots_api::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use telegram_bots_api::api::structs::giveaway::Giveaway;
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted;
use telegram_bots_api::api::structs::giveaway_created::GiveawayCreated;
use telegram_bots_api::api::structs::giveaway_winners::GiveawayWinners;
use telegram_bots_api::api::structs::invoice::Invoice;
use telegram_bots_api::api::structs::link_preview_options::LinkPreviewOptions;
use telegram_bots_api::api::structs::location::Location;
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_bots_api::api::structs::passport_data::PassportData;
use telegram_bots_api::api::structs::photo_size::PhotoSize;
use telegram_bots_api::api::structs::poll::Poll;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered;
use telegram_bots_api::api::structs::sticker::Sticker;
use telegram_bots_api::api::structs::story::Story;
use telegram_bots_api::api::structs::successful_payment::SuccessfulPayment;
use telegram_bots_api::api::structs::user::User;
use telegram_bots_api::api::structs::users_shared::UsersShared;
use telegram_bots_api::api::structs::venue::Venue;
use telegram_bots_api::api::structs::video::Video;
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted;
use telegram_bots_api::api::structs::video_note::VideoNote;
use telegram_bots_api::api::structs::voice::Voice;
use telegram_bots_api::api::structs::web_app_data::WebAppData;
use telegram_bots_api::api::structs::write_access_allowed::WriteAccessAllowed;

#[derive(Debug)]
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
    Dice(DiceMessage),
    /// Message is a game, information about the game.
    Game(GameMessage),
    /// Message is a native poll, information about the poll
    Poll(PollMessage),
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    Venue(VenueMessage),
    ///  Message is a shared location, information about the location
    Location(LocationMessage),
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
    ChannelChatCreated(ChannelChatCreatedMessage),
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
    PinnedMessage(MaybeInaccessibleMessage),
    /// Message is an invoice for a payment, information about the invoice.
    Invoice(InvoiceMessage),
    /// Message is a service message about a successful payment, information about the payment.
    SuccessfulPayment(SuccessfulPaymentMessage),
    /// Service message: users were shared with the bot
    UsersShared(UsersSharedMessage),
    /// Service message: a chat was shared with the bot
    ChatShared(ChatSharedMessage),
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
    ChatBoostAdded(ChatBoostAddedMessage),
    /// Service message: chat background set
    ChatBackground(ChatBackgroundMessage),
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
    Unexpected,
}

impl From<Inner> for MessageKind {
    fn from(_inner: Inner) -> Self {
        // TODO: match
        Self::Unexpected
    }
}

#[derive(Debug)]
pub struct TextMessage {
    pub entities: Vec<MessageEntity>,
    pub text: String,
    pub link_preview_options: LinkPreviewOptions,
}

#[derive(Debug)]
pub struct CommandMessage {
    pub text: String,
}

#[derive(Debug)]
pub struct AnimationMessage {
    pub audio: Animation,
    pub document: Document,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct AudioMessage {
    pub audio: Audio,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct DocumentMessage {
    pub document: Document,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct PhotoMessage {
    pub photo: Vec<PhotoSize>,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct StickerMessage {
    pub sticker: Sticker,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

#[derive(Debug)]
pub struct StoryMessage {
    pub story: Story,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

#[derive(Debug)]
pub struct VideoMessage {
    pub video: Video,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct VideoNoteMessage {
    pub video_note: VideoNote,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct VoiceMessage {
    pub voice: Voice,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug)]
pub struct ContactMessage {
    pub contact: Contact,
}

#[derive(Debug)]
pub struct DiceMessage {
    pub dice: Dice,
}

#[derive(Debug)]
pub struct GameMessage {
    pub game: Game,
}

#[derive(Debug)]
pub struct PollMessage {
    pub poll: Poll,
}

#[derive(Debug)]
pub struct VenueMessage {
    pub venue: Venue,
}

#[derive(Debug)]
pub struct LocationMessage {
    pub location: Location,
}

#[derive(Debug)]
pub struct NewChatMembersMessage {
    pub new_chat_members: Vec<User>,
}

#[derive(Debug)]
pub struct LeftChatMemberMessage {
    pub left_chat_member: User,
}

#[derive(Debug)]
pub struct NewChatTitleMessage {
    pub new_chat_title: String,
}

#[derive(Debug)]
pub struct NewChatPhotoMessage {
    pub new_chat_photo: Vec<PhotoSize>,
}

#[derive(Debug)]
pub struct DeleteChatPhotoMessage {
    pub delete_chat_photo: bool,
}

#[derive(Debug)]
pub struct GroupChatCreatedMessage {
    pub group_chat_created: bool,
}

#[derive(Debug)]
pub struct SupergroupChatCreatedMessage {
    pub supergroup_chat_created: bool,
}

#[derive(Debug)]
pub struct ChannelChatCreatedMessage {
    pub channel_chat_created: bool,
}

#[derive(Debug)]
pub struct MessageAutoDeleteTimerChangedMessage {
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

#[derive(Debug)]
pub struct MigrateToChatMessage {
    pub migrate_to_chat_id: i64,
}

#[derive(Debug)]
pub struct MigrateFromChatMessage {
    pub migrate_from_chat_id: i64,
}

#[derive(Debug)]
pub struct PinnedMessage {
    pub pinned_message: MaybeInaccessibleMessage,
}

#[derive(Debug)]
pub struct InvoiceMessage {
    pub invoice: Invoice,
}

#[derive(Debug)]
pub struct SuccessfulPaymentMessage {
    pub successful_payment: SuccessfulPayment,
}

#[derive(Debug)]
pub struct UsersSharedMessage {
    pub users_shared: UsersShared,
}

#[derive(Debug)]
pub struct ChatSharedMessage {
    pub chat_shared: ChatShared,
}

#[derive(Debug)]
pub struct ConnectedWebsiteMessage {
    pub connected_website: String,
}

#[derive(Debug)]
pub struct WriteAccessAllowedMessage {
    pub write_access_allowed: WriteAccessAllowed,
}

#[derive(Debug)]
pub struct PassportDataMessage {
    pub passport_data: PassportData,
}

#[derive(Debug)]
pub struct ProximityAlertTriggeredMessage {
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

#[derive(Debug)]
pub struct ChatBoostAddedMessage {
    pub boost_added: ChatBoostAdded,
}

#[derive(Debug)]
pub struct ChatBackgroundMessage {
    pub chat_background_set: ChatBackground,
}

#[derive(Debug)]
pub struct ForumTopicCreatedMessage {
    pub forum_topic_edited: ForumTopicCreated,
}

#[derive(Debug)]
pub struct ForumTopicEditedMessage {
    pub forum_topic_created: ForumTopicEdited,
}

#[derive(Debug)]
pub struct ForumTopicClosedMessage {
    pub forum_topic_closed: ForumTopicClosed,
}

#[derive(Debug)]
pub struct ForumTopicReopenedMessage {
    pub forum_topic_reopened: ForumTopicReopened,
}

#[derive(Debug)]
pub struct GeneralForumTopicHiddenMessage {
    pub general_forum_topic_hidden: GeneralForumTopicHidden,
}

#[derive(Debug)]
pub struct GeneralForumTopicUnhiddenMessage {
    pub general_forum_topic_unhidden: GeneralForumTopicUnhidden,
}

#[derive(Debug)]
pub struct GiveawayCreatedMessage {
    pub giveaway_created: GiveawayCreated,
}

#[derive(Debug)]
pub struct GiveawayMessage {
    pub giveaway: Giveaway,
}

#[derive(Debug)]
pub struct GiveawayWinnersMessage {
    pub giveaway_winners: GiveawayWinners,
}

#[derive(Debug)]
pub struct GiveawayCompletedMessage {
    pub giveaway_completed: GiveawayCompleted,
}

#[derive(Debug)]
pub struct VideoChatScheduledMessage {
    pub video_chat_scheduled: VideoChatScheduled,
}

#[derive(Debug)]
pub struct VideoChatStartedMessage {
    pub video_chat_started: VideoChatStarted,
}

#[derive(Debug)]
pub struct VideoChatEndedMessage {
    pub video_chat_ended: VideoChatEnded,
}

#[derive(Debug)]
pub struct VideoChatParticipantsInvitedMessage {
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}

#[derive(Debug)]
pub struct WebAppDataMessage {
    pub web_app_data: WebAppData,
}
