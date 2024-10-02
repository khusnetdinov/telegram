use crate::traits::storage::Storage;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct MemoryStorage<State> {
    pub states: HashMap<i64, State>,
}

impl<State> MemoryStorage<State> {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            states: HashMap::new(),
        }))
    }
}

impl<State> Storage<State> for MemoryStorage<State>
where
    State: Debug + Clone + Send + 'static,
{
    type Error = Box<dyn std::error::Error>;
}
