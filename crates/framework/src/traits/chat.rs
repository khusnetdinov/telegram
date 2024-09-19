#[async_trait::async_trait]
pub trait Chat {
    // async fn approve_chat_join_request(
    //     &self,
    //     params: &ApproveChatJoinRequest,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn ban_chat_member(
    //     &self,
    //     params: &BanChatMember,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn ban_chat_sender_chat(
    //     &self,
    //     params: &BanChatSenderChat,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn create_chat_invite_link(
    //     &self,
    //     params: &CreateChatInviteLink,
    // ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;
    //
    // async fn decline_chat_join_request(
    //     &self,
    //     params: &DeclineChatJoinRequest,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn edit_chat_invite_link(
    //     &self,
    //     params: &EditChatInviteLink,
    // ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;
    //
    // async fn export_chat_invite_link(
    //     &self,
    //     params: &ExportChatInviteLink,
    // ) -> Result<String, Box<dyn std::error::Error>>;
    //
    // async fn get_chat(&self, params: &GetChat) -> Result<ChatFullInfo, Box<dyn std::error::Error>>;
    //
    // async fn get_chat_member(
    //     &self,
    //     params: &GetChatMember,
    // ) -> Result<ChatMember, Box<dyn std::error::Error>>;
    //
    // async fn get_user_chat_boosts(
    //     &self,
    //     params: &GetUserChatBoosts,
    // ) -> Result<UserChatBoosts, Box<dyn std::error::Error>>;
    //
    // async fn leave_chat(&self, params: &LeaveChat) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn promote_chat_member(
    //     &self,
    //     params: &PromoteChatMember,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn restrict_chat_member(
    //     &self,
    //     params: &RestrictChatMember,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn send_chat_action(
    //     &self,
    //     params: &SendChatAction,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn set_chat_administrator_custom_title(
    //     &self,
    //     params: &SetChatAdministratorCustomTitle,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn set_chat_photo(
    //     &self,
    //     params: &SetChatPhoto,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn unban_chat_member(
    //     &self,
    //     params: &UnbanChatMember,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn unban_chat_sender_chat(
    //     &self,
    //     params: &UnbanChatSenderChat,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn unpin_all_chat_messages(
    //     &self,
    //     params: &UnpinAllChatMessages,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    // async fn unpin_chat_message(
    //     &self,
    //     params: &UnpinChatMessage,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn delete_chat_photo(
    //     &self,
    //     params: &DeleteChatPhoto,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    // async fn get_chat_administrators(
    //     &self,
    //     params: &GetChatAdministrators,
    // ) -> Result<Vec<ChatMember>, Box<dyn std::error::Error>>;
    //
    // async fn pin_chat_message(
    //     &self,
    //     params: &PinChatMessage,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    // async fn revoke_chat_invite_link(
    //     &self,
    //     params: &RevokeChatInviteLink,
    // ) -> Result<ChatInviteLink, Box<dyn std::error::Error>>;
    //
    // async fn set_chat_description(
    //     &self,
    //     params: &SetChatDescription,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    // async fn set_chat_title(
    //     &self,
    //     params: &SetChatTitle,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
    //
    // async fn get_chat_member_count(
    //     &self,
    //     params: &GetChatMemberCount,
    // ) -> Result<i64, Box<dyn std::error::Error>>;
    //
    // async fn set_chat_permissions(
    //     &self,
    //     params: &SetChatPermissions,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
}
