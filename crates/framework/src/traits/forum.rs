use crate::structs::forum_topics::forum_topic::ForumTopic;
use std::error::Error;

#[async_trait::async_trait]
pub trait Forum {
    async fn create_forum_topic(
        &self,
        chat_id: i64,
        name: String,
        icon_color: Option<i64>,
        icon_custom_emoji_id: Option<String>,
    ) -> Result<ForumTopic, Box<dyn Error>>;

    async fn edit_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
        name: Option<String>,
        icon_custom_emoji_id: Option<String>,
    ) -> Result<bool, Box<dyn Error>>;

    async fn close_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn Error>>;

    async fn reopen_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn Error>>;

    async fn delete_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn Error>>;

    async fn unpin_all_forum_topic_messages(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> Result<bool, Box<dyn Error>>;

    async fn edit_general_forum_topic(
        &self,
        chat_id: i64,
        name: String,
    ) -> Result<bool, Box<dyn Error>>;

    async fn close_general_forum_topic(&self, chat_id: i64) -> Result<bool, Box<dyn Error>>;

    async fn reopen_general_forum_topic(&self, chat_id: i64) -> Result<bool, Box<dyn Error>>;

    async fn hide_general_forum_topic(&self, chat_id: i64) -> Result<bool, Box<dyn Error>>;

    async fn unhide_general_forum_topic(&self, chat_id: i64) -> Result<bool, Box<dyn Error>>;

    async fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: i64,
    ) -> Result<bool, Box<dyn Error>>;
}
