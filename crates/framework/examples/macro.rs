use telegram_bots_api::api::structs::chat::Chat;
use telegram_bots_api::api::structs::giveaway_winners::GiveawayWinners as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i64,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub winners: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}
