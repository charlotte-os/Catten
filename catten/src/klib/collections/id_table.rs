use alloc::sync::Arc;
use alloc::vec::Vec;

use hashbrown::HashMap;
use spin::{Mutex, RwLock};

pub struct IdTable<I, T>
where
    I: From<usize> + Copy + core::cmp::Eq + core::hash::Hash,
{
    map: HashMap<I, Arc<RwLock<T>>>,
    available_ids: Mutex<Vec<I>>,
}

impl<I, T> IdTable<I, T>
where
    I: From<usize> + Copy + core::cmp::Eq + core::hash::Hash,
{
    pub fn new() -> Self {
        IdTable {
            map: HashMap::new(),
            available_ids: Mutex::new(Vec::new()),
        }
    }

    pub fn add_element(&mut self, element: T) -> I {
        let element_id = {
            if let Some(id) = self.available_ids.lock().pop() {
                id
            } else {
                self.map.len().into()
            }
        };
        self.map.insert(element_id, Arc::new(RwLock::new(element)));
        element_id
    }

    pub fn try_get_element_arc(&self, element_id: I) -> Option<Arc<RwLock<T>>> {
        if let Some(lock_ptr) = self.map.get(&element_id) {
            Some(lock_ptr.clone())
        } else {
            None
        }
    }

    pub fn remove_element(&mut self, element_id: I) {
        self.map.remove(&element_id);
        self.available_ids.lock().push(element_id);
    }
}

unsafe impl<I, T> Send for IdTable<I, T>
where
    I: From<usize> + Copy + core::cmp::Eq + core::hash::Hash,
    T: Send,
{
}
