pub trait Storage<State> {
    type Error;

    fn get(&'static self, chat_id: i64) -> Option<&'static State>;

    fn set(&mut self, chat_id: i64, state: State) -> Option<State>;
}
