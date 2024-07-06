use crate::config::Config;
use crate::structs::user::User;
use crate::structs::webhook::Webhook;
use std::fmt::Debug;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;

#[derive(Debug, Clone)]
pub struct BotsApi {
    pub config: Config,
    pub client: Async,
    pub user: User,
    pub webhook: Webhook,
}

impl BotsApi {
    pub async fn new(
        config: Config,
        client: Async,
        user: User,
        webhook: Webhook,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config,
            client,
            user,
            webhook,
        })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new();
        let client = Async::new(
            config.timeout,
            config.connect_timeout,
            config.url.as_str(),
            config.token.as_str(),
        );

        let user = User::from(client.get_me().await?);
        let webhook = Webhook::from(&config);

        let bots_api = Self::new(config, client, user, webhook).await?;

        Ok(bots_api)
    }
}

// #[async_trait::async_trait]
// impl Sender for BotsApi {
//     async fn send_chat_action(
//         &self,
//         chat_id: i64,
//         action: ChatAction,
//         options: Option<SendOptions>,
//     ) -> Result<bool, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendChatAction {
//                 action: action.into(),
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 message_thread_id: options.message_thread_id,
//             }
//         } else {
//             SendChatAction {
//                 action: action.into(),
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_chat_action(&params).await?)
//     }

//     async fn send_contact(
//         &self,
//         chat_id: i64,
//         phone_number: String,

//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendContact {
//                 phone_number,
//                 first_name,
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//                 last_name: options.last_name,
//                 vcard: options.vcard,
//             }
//         } else {
//             SendContact {
//                 phone_number,
//                 first_name,
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_contact(&params).await?.into())
//     }

//     async fn send_dice(
//         &self,
//         chat_id: i64,
//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendDice {
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 emoji: options.emoji.map(|emoji| emoji.into()),
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//             }
//         } else {
//             SendDice {
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_dice(&params).await?.into())
//     }

//     async fn send_game(
//         &self,
//         chat_id: i64,
//         game_short_name: String,
//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendGame {
//                 game_short_name,
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//             }
//         } else {
//             SendGame {
//                 game_short_name,
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_game(&params).await?.into())
//     }

//     async fn send_message(
//         &self,
//         chat_id: i64,
//         text: String,
//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendMessage {
//                 text,
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//                 parse_mode: options.parse_mode,
//                 entities: options.entities,
//                 link_preview_options: options.link_preview_options,
//             }
//         } else {
//             SendMessage {
//                 text,
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_message(&params).await?.into())
//     }

//     async fn send_poll(
//         &self,
//         chat_id: i64,
//         question: String,
//         poll_options: Vec<InputPollOption>,
//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendPoll {
//                 question,
//                 options: poll_options,
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//                 allows_multiple_answers: options.allows_multiple_answers,
//                 is_anonymous: options.is_anonymous,
//                 correct_option_id: options.correct_option_id,
//                 explanation: options.explanation,
//                 explanation_parse_mode: options.explanation_parse_mode,
//                 explanation_entities: options.explanation_entities,
//                 open_period: options.open_period,
//                 close_date: options.close_date,
//                 is_closed: options.is_closed,
//                 question_entities: options.question_entities,
//                 question_parse_mode: options.question_parse_mode,
//                 kind: options.kind,
//             }
//         } else {
//             SendPoll {
//                 question,
//                 options: poll_options,
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_poll(&params).await?.into())
//     }

//     async fn send_venue(
//         &self,
//         chat_id: i64,
//         latitude: f64,
//         longitude: f64,
//         title: String,
//         address: String,
//         options: Option<SendOptions>,
//     ) -> Result<Message, Box<dyn std::error::Error>> {
//         let params = if let Some(options) = options {
//             SendVenue {
//                 latitude,
//                 longitude,
//                 title,
//                 address,
//                 chat_id: ChatUId::from(chat_id),
//                 business_connection_id: options.business_connection_id,
//                 disable_notification: options.disable_notification,
//                 protect_content: options.protect_content,
//                 message_effect_id: options.message_effect_id,
//                 message_thread_id: options.message_thread_id,
//                 reply_parameters: options.reply_parameters,
//                 reply_markup: options.reply_markup,
//                 foursquare_id: options.foursquare_id,
//                 foursquare_type: options.foursquare_type,
//                 google_place_id: options.google_place_id,
//                 google_place_type: options.google_place_type,
//             }
//         } else {
//             SendVenue {
//                 latitude,
//                 longitude,
//                 title,
//                 address,
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             }
//         };

//         Ok(self.client.send_venue(&params).await?.into())
//     }
// }
