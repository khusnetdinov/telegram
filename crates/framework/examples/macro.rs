// use telegram_bots_api::api::structs::chat_invite_link::ChatInviteLink as Remote;
// use telegram_bots_api::api::structs::user::User;
// use telegram_macros::FromRemote;
//
// #[derive(FromRemote)]
// pub struct ChatInviteLink {
//     pub invite_link: String,
//     pub creator: User,
//     pub creates_join_request: bool,
//     pub is_primary: bool,
//     pub is_revoked: bool,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     pub name: Option<String>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     pub expire_date: Option<i64>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     pub member_limit: Option<i64>,
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     pub pending_join_request_count: Option<i64>,
// }
