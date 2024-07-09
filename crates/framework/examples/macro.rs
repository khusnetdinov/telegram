use telegram_bots_api::api::enums::chat_boost_source::ChatBoostSource;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}
