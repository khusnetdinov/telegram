use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway::Giveaway as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
}
impl From<Remote> for Giveaway {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(map, into)]
            chats: remote
                .chats
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            winners_selection_date: remote.winners_selection_date,
            winner_count: remote.winner_count,
            only_new_members: remote.only_new_members,
            has_public_winners: remote.has_public_winners,
            prize_description: remote.prize_description,
            country_codes: remote.country_codes,
            premium_subscription_month_count: remote.premium_subscription_month_count,
        }
    }
}
impl From<Message> for Giveaway {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway: Some(giveaway),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(giveaway)
    }
}
