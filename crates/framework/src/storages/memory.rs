use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct MemoryStorage<ST> {
    pub states: Mutex<HashMap<i64, ST>>,
}

impl<ST> MemoryStorage<ST> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            states: Mutex::new(HashMap::new()),
        })
    }
}
