use crate::traits::storage::Storage;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MemoryStorage<ST> {
    pub states: RefCell<HashMap<i64, ST>>,
}

impl<ST: Debug + Clone> MemoryStorage<ST> {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            states: RefCell::new(HashMap::new()),
        })
    }
}

impl<ST: Debug + Clone> Storage<ST> for MemoryStorage<ST> {
    fn get(&self, chat_id: i64) -> Option<ST> {
        self.states.borrow().get(&chat_id).cloned()
    }

    fn set(&self, chat_id: i64, state: ST) {
        self.states.borrow_mut().insert(chat_id, state);
    }
}
