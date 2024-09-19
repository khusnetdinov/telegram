use crate::bots_api::BotsApi;
use crate::enums::chat_member::ChatMember;
use crate::structs::chats::chat_full_info::ChatFullInfo;
use crate::structs::chats::chat_invite_link::ChatInviteLink;
use crate::structs::chats::chat_permissions::ChatPermissions;
use crate::structs::chats::options::Options as ChatOptions;
use crate::structs::chats::user_chat_boosts::UserChatBoosts;
use crate::structs::input_file::InputFile;
use crate::structs::message_id::MessageId;
use crate::traits::chat::Chat;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::approve_chat_join_request::ApproveChatJoinRequest;
use telegram_bots_api::api::params::ban_chat_member::BanChatMember;
use telegram_bots_api::api::params::ban_chat_sender_chat::BanChatSenderChat;
use telegram_bots_api::api::params::create_chat_invite_link::CreateChatInviteLink;
use telegram_bots_api::api::params::decline_chat_join_request::DeclineChatJoinRequest;
use telegram_bots_api::api::params::delete_chat_photo::DeleteChatPhoto;
use telegram_bots_api::api::params::edit_chat_invite_link::EditChatInviteLink;
use telegram_bots_api::api::params::export_chat_invite_link::ExportChatInviteLink;
use telegram_bots_api::api::params::get_chat::GetChat;
use telegram_bots_api::api::params::get_chat_administrators::GetChatAdministrators;
use telegram_bots_api::api::params::get_chat_member::GetChatMember;
use telegram_bots_api::api::params::get_chat_member_count::GetChatMemberCount;
use telegram_bots_api::api::params::get_user_chat_boosts::GetUserChatBoosts;
use telegram_bots_api::api::params::leave_chat::LeaveChat;
use telegram_bots_api::api::params::pin_chat_message::PinChatMessage;
use telegram_bots_api::api::params::promote_chat_member::PromoteChatMember;
use telegram_bots_api::api::params::restrict_chat_member::RestrictChatMember;
use telegram_bots_api::api::params::revoke_chat_invite_link::RevokeChatInviteLink;
use telegram_bots_api::api::params::set_chat_administrator_custom_title::SetChatAdministratorCustomTitle;
use telegram_bots_api::api::params::set_chat_description::SetChatDescription;
use telegram_bots_api::api::params::set_chat_permissions::SetChatPermissions;
use telegram_bots_api::api::params::set_chat_photo::SetChatPhoto;
use telegram_bots_api::api::params::set_chat_title::SetChatTitle;
use telegram_bots_api::api::params::unban_chat_member::UnbanChatMember;
use telegram_bots_api::api::params::unban_chat_sender_chat::UnbanChatSenderChat;
use telegram_bots_api::api::params::unpin_all_chat_messages::UnpinAllChatMessages;
use telegram_bots_api::api::params::unpin_chat_message::UnpinChatMessage;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Chat for BotsApi {
    async fn approve_chat_join_request(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = ApproveChatJoinRequest {
            chat_id: ChatUId::from(chat_id),
            user_id,
        };

        Ok(self.client.approve_chat_join_request(&params).await?)
    }

    async fn ban_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let ChatOptions {
            until_date,
            revoke_messages,
            ..
        } = chat_options;

        let params = BanChatMember {
            chat_id: ChatUId::from(chat_id),
            user_id,
            until_date,
            revoke_messages,
        };

        Ok(self.client.ban_chat_member(&params).await?)
    }

    async fn ban_chat_sender_chat(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = BanChatSenderChat {
            chat_id: ChatUId::from(chat_id),
            sender_chat_id: ChatUId::from(sender_chat_id),
        };

        Ok(self.client.ban_chat_sender_chat(&params).await?)
    }

    async fn create_chat_invite_link(
        &self,
        chat_id: i64,
        chat_options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let ChatOptions {
            name,
            expire_date,
            member_limit,
            creates_join_request,
            ..
        } = chat_options;

        let params = CreateChatInviteLink {
            chat_id: ChatUId::from(chat_id),
            name,
            expire_date,
            member_limit,
            creates_join_request,
        };

        Ok(self.client.create_chat_invite_link(&params).await?.into())
    }

    async fn decline_chat_join_request(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeclineChatJoinRequest {
            chat_id: ChatUId::from(chat_id),
            user_id,
        };

        Ok(self.client.decline_chat_join_request(&params).await?)
    }

    async fn edit_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
        chat_options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let ChatOptions {
            name,
            expire_date,
            member_limit,
            creates_join_request,
            ..
        } = chat_options;

        let params = EditChatInviteLink {
            chat_id: ChatUId::from(chat_id),
            invite_link,
            name,
            expire_date,
            member_limit,
            creates_join_request,
        };

        Ok(self.client.edit_chat_invite_link(&params).await?.into())
    }

    async fn export_chat_invite_link(
        &self,
        chat_id: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = ExportChatInviteLink {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.export_chat_invite_link(&params).await?)
    }

    async fn get_chat(&self, chat_id: i64) -> Result<ChatFullInfo, Box<dyn std::error::Error>> {
        let params = GetChat {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.get_chat(&params).await?.into())
    }

    async fn get_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<ChatMember, Box<dyn std::error::Error>> {
        let params = GetChatMember {
            chat_id: ChatUId::from(chat_id),
            user_id,
        };

        Ok(self.client.get_chat_member(&params).await?.into())
    }

    async fn get_user_chat_boosts(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<UserChatBoosts, Box<dyn std::error::Error>> {
        let params = GetUserChatBoosts {
            chat_id: ChatUId::from(chat_id),
            user_id,
        };

        Ok(self.client.get_user_chat_boosts(&params).await?.into())
    }

    async fn leave_chat(&self, chat_id: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let params = LeaveChat {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.leave_chat(&params).await?)
    }

    async fn promote_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let ChatOptions {
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages,
            can_edit_messages,
            can_pin_messages,
            can_post_stories,
            can_edit_stories,
            can_delete_stories,
            can_manage_topics,
            ..
        } = chat_options;

        let params = PromoteChatMember {
            chat_id: ChatUId::from(chat_id),
            user_id,
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages,
            can_edit_messages,
            can_pin_messages,
            can_post_stories,
            can_edit_stories,
            can_delete_stories,
            can_manage_topics,
        };

        Ok(self.client.promote_chat_member(&params).await?)
    }

    async fn restrict_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        permissions: ChatPermissions,
        chat_options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let ChatOptions {
            until_date,
            use_independent_chat_permissions,
            ..
        } = chat_options;

        let params = RestrictChatMember {
            chat_id: ChatUId::from(chat_id),
            user_id,
            permissions: permissions.into(),
            until_date,
            use_independent_chat_permissions,
        };

        Ok(self.client.restrict_chat_member(&params).await?)
    }

    async fn set_chat_administrator_custom_title(
        &self,
        chat_id: i64,
        user_id: i64,
        custom_title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatAdministratorCustomTitle {
            chat_id: ChatUId::from(chat_id),
            user_id,
            custom_title,
        };

        Ok(self
            .client
            .set_chat_administrator_custom_title(&params)
            .await?)
    }

    async fn set_chat_photo(
        &self,
        chat_id: i64,
        photo: InputFile,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatPhoto {
            chat_id: ChatUId::from(chat_id),
            photo: photo.into(),
        };

        Ok(self.client.set_chat_photo(&params).await?)
    }

    async fn unban_chat_member(
        &self,
        chat_id: i64,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnbanChatMember {
            chat_id: ChatUId::from(chat_id),
            user_id,
            only_if_banned,
        };

        Ok(self.client.unban_chat_member(&params).await?)
    }

    async fn unban_chat_sender_chat(
        &self,
        chat_id: i64,
        sender_chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnbanChatSenderChat {
            chat_id: ChatUId::from(chat_id),
            sender_chat_id: ChatUId::from(sender_chat_id),
        };

        Ok(self.client.unban_chat_sender_chat(&params).await?)
    }

    async fn unpin_all_chat_messages(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinAllChatMessages {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.unpin_all_chat_messages(&params).await?)
    }

    async fn unpin_chat_message(
        &self,
        chat_id: i64,
        message_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinChatMessage {
            chat_id: ChatUId::from(chat_id),
            message_id,
            business_connection_id,
        };

        Ok(self.client.unpin_chat_message(&params).await?)
    }

    async fn delete_chat_photo(&self, chat_id: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteChatPhoto {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.delete_chat_photo(&params).await?)
    }

    async fn get_chat_administrators(
        &self,
        chat_id: i64,
    ) -> Result<Vec<ChatMember>, Box<dyn std::error::Error>> {
        let params = GetChatAdministrators {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self
            .client
            .get_chat_administrators(&params)
            .await?
            .iter()
            .map(|inner| inner.clone().into())
            .collect())
    }

    async fn pin_chat_message(
        &self,
        chat_id: i64,
        message_id: MessageId,
        business_connection_id: Option<String>,
        disable_notification: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = PinChatMessage {
            chat_id: ChatUId::from(chat_id),
            message_id: message_id.into(),
            business_connection_id,
            disable_notification,
        };

        Ok(self.client.pin_chat_message(&params).await?)
    }

    async fn revoke_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let params = RevokeChatInviteLink {
            chat_id: ChatUId::from(chat_id),
            invite_link,
        };

        Ok(self.client.revoke_chat_invite_link(&params).await?.into())
    }

    async fn set_chat_description(
        &self,
        chat_id: i64,
        description: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatDescription {
            chat_id: ChatUId::from(chat_id),
            description,
        };

        Ok(self.client.set_chat_description(&params).await?)
    }

    async fn set_chat_title(
        &self,
        chat_id: i64,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatTitle {
            chat_id: ChatUId::from(chat_id),
            title,
        };

        Ok(self.client.set_chat_title(&params).await?)
    }

    async fn get_chat_member_count(&self, chat_id: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let params = GetChatMemberCount {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.get_chat_member_count(&params).await?)
    }

    async fn set_chat_permissions(
        &self,
        chat_id: i64,
        permissions: ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatPermissions {
            chat_id: ChatUId::from(chat_id),
            permissions: permissions.into(),
            use_independent_chat_permissions,
        };

        Ok(self.client.set_chat_permissions(&params).await?)
    }
}
