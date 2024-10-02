use crate::traits::storage::Storage;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct MemoryStorage<State> {
    states: HashMap<i64, State>,
}

impl<State> MemoryStorage<State> {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }
}

impl<State> Storage<State> for MemoryStorage<State>
where
    State: Debug + Clone + Send + 'static,
{
    type Error = Box<dyn std::error::Error>;

    fn get(&'static self, chat_id: i64) -> Option<&'static State> {
        self.states.get(&chat_id)
    }

    fn set(&mut self, chat_id: i64, state: State) -> Option<State> {
        self.states.insert(chat_id, state)
    }
}
