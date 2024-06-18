pub trait Storage<ST> {
    fn get(&self, chat_id: i64) -> Option<ST>;
    fn set(&self, chat_id: i64, state: ST);
}
