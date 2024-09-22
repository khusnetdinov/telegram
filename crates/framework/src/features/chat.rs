use crate::bots_api::BotsApi;
use crate::enums::chat_member::ChatMember;
use crate::enums::chat_uid::ChatUId;
use crate::structs::chats::chat_full_info::ChatFullInfo;
use crate::structs::chats::chat_invite_link::ChatInviteLink;
use crate::structs::chats::chat_permissions::ChatPermissions;
use crate::structs::chats::options::Options as ChatOptions;
use crate::structs::input_file::InputFile;
use crate::structs::messages::message_id::MessageId;
use crate::structs::users::user_chat_boosts::UserChatBoosts;
use crate::traits::features::chat::Chat;
use telegram_bots_api::api::params::approve_chat_join_request::ApproveChatJoinRequest;
use telegram_bots_api::api::params::ban_chat_member::BanChatMember;
use telegram_bots_api::api::params::ban_chat_sender_chat::BanChatSenderChat;
use telegram_bots_api::api::params::create_chat_invite_link::CreateChatInviteLink;
use telegram_bots_api::api::params::create_chat_subscription_invite_link::CreateChatSubscriptionInviteLink;
use telegram_bots_api::api::params::decline_chat_join_request::DeclineChatJoinRequest;
use telegram_bots_api::api::params::delete_chat_photo::DeleteChatPhoto;
use telegram_bots_api::api::params::edit_chat_invite_link::EditChatInviteLink;
use telegram_bots_api::api::params::edit_chat_subscription_invite_link::EditChatSubscriptionInviteLink;
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
        chat_id: ChatUId,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = ApproveChatJoinRequest {
            chat_id: chat_id.into(),
            user_id,
        };

        Ok(self.client.approve_chat_join_request(&params).await?)
    }

    async fn ban_chat_member(
        &self,
        chat_id: ChatUId,
        user_id: i64,
        options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let ChatOptions {
            until_date,
            revoke_messages,
            ..
        } = options;

        let params = BanChatMember {
            chat_id: chat_id.into(),
            user_id,
            until_date,
            revoke_messages,
        };

        Ok(self.client.ban_chat_member(&params).await?)
    }

    async fn ban_chat_sender_chat(
        &self,
        chat_id: ChatUId,
        sender_chat_id: ChatUId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = BanChatSenderChat {
            chat_id: chat_id.into(),
            sender_chat_id: sender_chat_id.into(),
        };

        Ok(self.client.ban_chat_sender_chat(&params).await?)
    }

    async fn create_chat_invite_link(
        &self,
        chat_id: ChatUId,
        options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let ChatOptions {
            name,
            expire_date,
            member_limit,
            creates_join_request,
            ..
        } = options;

        let params = CreateChatInviteLink {
            chat_id: chat_id.into(),
            name,
            expire_date,
            member_limit,
            creates_join_request,
        };

        Ok(self.client.create_chat_invite_link(&params).await?.into())
    }

    async fn decline_chat_join_request(
        &self,
        chat_id: ChatUId,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeclineChatJoinRequest {
            chat_id: chat_id.into(),
            user_id,
        };

        Ok(self.client.decline_chat_join_request(&params).await?)
    }

    async fn edit_chat_invite_link(
        &self,
        chat_id: ChatUId,
        invite_link: String,
        options: ChatOptions,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let ChatOptions {
            name,
            expire_date,
            member_limit,
            creates_join_request,
            ..
        } = options;

        let params = EditChatInviteLink {
            chat_id: chat_id.into(),
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
        chat_id: ChatUId,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = ExportChatInviteLink {
            chat_id: chat_id.into(),
        };

        Ok(self.client.export_chat_invite_link(&params).await?)
    }

    async fn get_chat(&self, chat_id: ChatUId) -> Result<ChatFullInfo, Box<dyn std::error::Error>> {
        let params = GetChat {
            chat_id: chat_id.into(),
        };

        Ok(self.client.get_chat(&params).await?.into())
    }

    async fn get_chat_member(
        &self,
        chat_id: ChatUId,
        user_id: i64,
    ) -> Result<ChatMember, Box<dyn std::error::Error>> {
        let params = GetChatMember {
            chat_id: chat_id.into(),
            user_id,
        };

        Ok(self.client.get_chat_member(&params).await?.into())
    }

    async fn get_user_chat_boosts(
        &self,
        chat_id: ChatUId,
        user_id: i64,
    ) -> Result<UserChatBoosts, Box<dyn std::error::Error>> {
        let params = GetUserChatBoosts {
            chat_id: chat_id.into(),
            user_id,
        };

        Ok(self.client.get_user_chat_boosts(&params).await?.into())
    }

    async fn leave_chat(&self, chat_id: ChatUId) -> Result<bool, Box<dyn std::error::Error>> {
        let params = LeaveChat {
            chat_id: chat_id.into(),
        };

        Ok(self.client.leave_chat(&params).await?)
    }

    async fn promote_chat_member(
        &self,
        chat_id: ChatUId,
        user_id: i64,
        options: ChatOptions,
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
        } = options;

        let params = PromoteChatMember {
            chat_id: chat_id.into(),
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
        chat_id: ChatUId,
        user_id: i64,
        permissions: ChatPermissions,
        options: ChatOptions,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let ChatOptions {
            until_date,
            use_independent_chat_permissions,
            ..
        } = options;

        let params = RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            permissions: permissions.into(),
            until_date,
            use_independent_chat_permissions,
        };

        Ok(self.client.restrict_chat_member(&params).await?)
    }

    async fn set_chat_administrator_custom_title(
        &self,
        chat_id: ChatUId,
        user_id: i64,
        custom_title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatAdministratorCustomTitle {
            chat_id: chat_id.into(),
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
        chat_id: ChatUId,
        photo: InputFile,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatPhoto {
            chat_id: chat_id.into(),
            photo: photo.into(),
        };

        Ok(self.client.set_chat_photo(&params).await?)
    }

    async fn unban_chat_member(
        &self,
        chat_id: ChatUId,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnbanChatMember {
            chat_id: chat_id.into(),
            user_id,
            only_if_banned,
        };

        Ok(self.client.unban_chat_member(&params).await?)
    }

    async fn unban_chat_sender_chat(
        &self,
        chat_id: ChatUId,
        sender_chat_id: ChatUId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnbanChatSenderChat {
            chat_id: chat_id.into(),
            sender_chat_id: sender_chat_id.into(),
        };

        Ok(self.client.unban_chat_sender_chat(&params).await?)
    }

    async fn unpin_all_chat_messages(
        &self,
        chat_id: ChatUId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinAllChatMessages {
            chat_id: chat_id.into(),
        };

        Ok(self.client.unpin_all_chat_messages(&params).await?)
    }

    async fn unpin_chat_message(
        &self,
        chat_id: ChatUId,
        message_id: Option<MessageId>,
        business_connection_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinChatMessage {
            chat_id: chat_id.into(),
            message_id: message_id.map(|inner| inner.into()),
            business_connection_id,
        };

        Ok(self.client.unpin_chat_message(&params).await?)
    }

    async fn delete_chat_photo(
        &self,
        chat_id: ChatUId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteChatPhoto {
            chat_id: chat_id.into(),
        };

        Ok(self.client.delete_chat_photo(&params).await?)
    }

    async fn get_chat_administrators(
        &self,
        chat_id: ChatUId,
    ) -> Result<Vec<ChatMember>, Box<dyn std::error::Error>> {
        let params = GetChatAdministrators {
            chat_id: chat_id.into(),
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
        chat_id: ChatUId,
        message_id: MessageId,
        business_connection_id: Option<String>,
        disable_notification: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = PinChatMessage {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            business_connection_id,
            disable_notification,
        };

        Ok(self.client.pin_chat_message(&params).await?)
    }

    async fn revoke_chat_invite_link(
        &self,
        chat_id: ChatUId,
        invite_link: String,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let params = RevokeChatInviteLink {
            chat_id: chat_id.into(),
            invite_link,
        };

        Ok(self.client.revoke_chat_invite_link(&params).await?.into())
    }

    async fn set_chat_description(
        &self,
        chat_id: ChatUId,
        description: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatDescription {
            chat_id: chat_id.into(),
            description,
        };

        Ok(self.client.set_chat_description(&params).await?)
    }

    async fn set_chat_title(
        &self,
        chat_id: ChatUId,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatTitle {
            chat_id: chat_id.into(),
            title,
        };

        Ok(self.client.set_chat_title(&params).await?)
    }

    async fn get_chat_member_count(
        &self,
        chat_id: ChatUId,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let params = GetChatMemberCount {
            chat_id: chat_id.into(),
        };

        Ok(self.client.get_chat_member_count(&params).await?)
    }

    async fn set_chat_permissions(
        &self,
        chat_id: ChatUId,
        permissions: ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetChatPermissions {
            chat_id: chat_id.into(),
            permissions: permissions.into(),
            use_independent_chat_permissions,
        };

        Ok(self.client.set_chat_permissions(&params).await?)
    }

    async fn create_chat_subscription_invite_link(
        &self,
        chat_id: ChatUId,
        subscription_period: i64,
        subscription_price: i64,
        name: Option<String>,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let params = CreateChatSubscriptionInviteLink {
            chat_id: chat_id.into(),
            subscription_period,
            subscription_price,
            name,
        };

        Ok(self
            .client
            .create_chat_subscription_invite_link(&params)
            .await?
            .into())
    }

    async fn edit_chat_subscription_invite_link(
        &self,
        chat_id: ChatUId,
        invite_link: String,
        name: Option<String>,
    ) -> Result<ChatInviteLink, Box<dyn std::error::Error>> {
        let params = EditChatSubscriptionInviteLink {
            chat_id: chat_id.into(),
            invite_link,
            name,
        };

        Ok(self
            .client
            .edit_chat_subscription_invite_link(&params)
            .await?
            .into())
    }
}
