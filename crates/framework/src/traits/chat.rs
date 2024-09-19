use crate::enums::chat_member::ChatMember;
use crate::structs::chats::chat_full_info::ChatFullInfo;
use crate::structs::chats::chat_invite_link::ChatInviteLink;
use crate::structs::chats::chat_permissions::ChatPermissions;
use crate::structs::chats::options::Options as ChatOptions;
use crate::structs::chats::user_chat_boosts::UserChatBoosts;
use crate::structs::input_file::InputFile;
use crate::structs::message_id::MessageId;

#[async_trait::async_trait]
pub trait Chat {
    async fn approve_chat_join_request(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn ban_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn ban_chat_sender_chat(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn create_chat_invite_link(
        &self,
        chat_id: i64,
        chat_options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;

    async fn decline_chat_join_request(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn edit_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
        chat_options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;

    async fn export_chat_invite_link(
        &self,
        chat_id: i64,
    ) -> Result<String, Box<dyn std::error::Error>>;

    async fn get_chat(&self, chat_id: i64) -> Result<ChatFullInfo, Box<dyn std::error::Error>>;

    async fn get_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<ChatMember, Box<dyn std::error::Error>>;

    async fn get_user_chat_boosts(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<UserChatBoosts, Box<dyn std::error::Error>>;

    async fn leave_chat(&self, chat_id: i64) -> Result<bool, Box<dyn std::error::Error>>;

    async fn promote_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn restrict_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        permissions: ChatPermissions,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_chat_administrator_custom_title(
        &self,
        chat_id: i64,
        user_id: i64,
        custom_title: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_chat_photo(
        &self,
        chat_id: i64,
        photo: InputFile,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn unban_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn unban_chat_sender_chat(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn unpin_all_chat_messages(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn unpin_chat_message(
        &self,
        chat_id: i64,
        message_id: Option<MessageId>,
        business_connection_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete_chat_photo(&self, chat_id: i64) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_chat_administrators(
        &self,
        chat_id: i64,
    ) -> Result<Vec<ChatMember>, Box<dyn std::error::Error>>;

    async fn pin_chat_message(
        &self,
        chat_id: i64,
        message_id: MessageId,
        business_connection_id: Option<String>,
        disable_notification: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn revoke_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;

    async fn set_chat_description(
        &self,
        chat_id: i64,
        description: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn set_chat_title(
        &self,
        chat_id: i64,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_chat_member_count(&self, chat_id: i64) -> Result<i64, Box<dyn std::error::Error>>;

    async fn set_chat_permissions(
        &self,
        chat_id: i64,
        permissions: ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
