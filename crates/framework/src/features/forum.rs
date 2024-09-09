use crate::bots_api::BotsApi;
use crate::structs::forum_topics::forum_topic::ForumTopic;
use crate::traits::forum::Forum;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::close_forum_topic::CloseForumTopic;
use telegram_bots_api::api::params::close_general_forum_topic::CloseGeneralForumTopic;
use telegram_bots_api::api::params::create_forum_topic::CreateForumTopic;
use telegram_bots_api::api::params::delete_forum_topic::DeleteForumTopic;
use telegram_bots_api::api::params::edit_forum_topic::EditForumTopic;
use telegram_bots_api::api::params::edit_general_forum_topic::EditGeneralForumTopic;
use telegram_bots_api::api::params::hide_general_forum_topic::HideGeneralForumTopic;
use telegram_bots_api::api::params::reopen_forum_topic::ReopenForumTopic;
use telegram_bots_api::api::params::reopen_general_forum_topic::ReopenGeneralForumTopic;
use telegram_bots_api::api::params::unhide_general_forum_topic::UnhideGeneralForumTopic;
use telegram_bots_api::api::params::unpin_all_forum_topic_messages::UnpinAllForumTopicMessages;
use telegram_bots_api::api::params::unpin_all_general_forum_topic_messages::UnpinAllGeneralForumTopicMessages;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Forum for BotsApi {
    async fn create_forum_topic(
        &self,
        chat_id: i64,
        name: String,
        icon_color: Option<i64>,
        icon_custom_emoji_id: Option<String>,
    ) -> Result<ForumTopic, Box<dyn std::error::Error>> {
        let params = CreateForumTopic {
            chat_id: ChatUId::from(chat_id),
            name,
            icon_color,
            icon_custom_emoji_id,
        };

        Ok(self.client.create_forum_topic(&params).await?.into())
    }

    async fn edit_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
        name: Option<String>,
        icon_custom_emoji_id: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = EditForumTopic {
            chat_id: ChatUId::from(chat_id),
            message_thread_id,
            name,
            icon_custom_emoji_id,
        };

        Ok(self.client.edit_forum_topic(&params).await?)
    }

    async fn close_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = CloseForumTopic {
            chat_id: ChatUId::from(chat_id),
            message_thread_id,
        };

        Ok(self.client.close_forum_topic(&params).await?)
    }

    async fn reopen_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = ReopenForumTopic {
            chat_id: ChatUId::from(chat_id),
            message_thread_id,
        };

        Ok(self.client.reopen_forum_topic(&params).await?)
    }

    async fn delete_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = DeleteForumTopic {
            chat_id: ChatUId::from(chat_id),
            message_thread_id,
        };

        Ok(self.client.delete_forum_topic(&params).await?)
    }

    async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinAllForumTopicMessages {
            chat_id: ChatUId::from(chat_id),
            message_thread_id,
        };

        Ok(self.client.unpin_all_forum_topic_messages(&params).await?)
    }

    async fn edit_general_forum_topic(
        &self,
        chat_id: i64,
        name: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = EditGeneralForumTopic {
            chat_id: ChatUId::from(chat_id),
            name,
        };

        Ok(self.client.edit_general_forum_topic(&params).await?)
    }

    async fn close_general_forum_topic(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = CloseGeneralForumTopic {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.close_general_forum_topic(&params).await?)
    }

    async fn reopen_general_forum_topic(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = ReopenGeneralForumTopic {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.reopen_general_forum_topic(&params).await?)
    }

    async fn hide_general_forum_topic(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = HideGeneralForumTopic {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.hide_general_forum_topic(&params).await?)
    }

    async fn unhide_general_forum_topic(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnhideGeneralForumTopic {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self.client.unhide_general_forum_topic(&params).await?)
    }

    async fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = UnpinAllGeneralForumTopicMessages {
            chat_id: ChatUId::from(chat_id),
        };

        Ok(self
            .client
            .unpin_all_general_forum_topic_messages(&params)
            .await?)
    }
}
