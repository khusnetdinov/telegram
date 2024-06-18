use crate::storages::memory::MemoryStorage;

#[derive(Debug)]
pub enum Storage<ST> {
    Memory(MemoryStorage<ST>),
}
