use telegram_bots_api::api::structs::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct MessageAutoDeleteTimerChangedMessage {
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}
