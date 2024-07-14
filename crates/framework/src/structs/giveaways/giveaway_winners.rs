use crate::structs::chat::Chat;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_winners::GiveawayWinners as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl From<Remote> for GiveawayWinners {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            giveaway_message_id: remote.giveaway_message_id,
            winners_selection_date: remote.winners_selection_date,
            winner_count: remote.winner_count,
            // TODO: #[remote(map, into)]
            winners: remote
                .winners
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            additional_chat_count: remote.additional_chat_count,
            premium_subscription_month_count: remote.premium_subscription_month_count,
            unclaimed_prize_count: remote.unclaimed_prize_count,
            only_new_members: remote.only_new_members,
            was_refunded: remote.was_refunded,
            prize_description: remote.prize_description,
        }
    }
}

impl From<Message> for GiveawayWinners {
    fn from(inner: Message) -> Self {
        let Message {
            giveaway_winners: Some(giveaway_winners),
            ..
        } = inner
        else {
            unreachable!()
        };

        Self::from(giveaway_winners)
    }
}
