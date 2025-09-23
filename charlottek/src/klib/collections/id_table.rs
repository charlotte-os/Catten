use alloc::vec::Vec;

use hashbrown::HashMap;

pub struct IdTable<I, T>
where
    I: From<usize> + Copy + core::cmp::Eq + core::hash::Hash,
{
    map: HashMap<I, T>,
    available_ids: Vec<I>,
}

impl<I, T> IdTable<I, T>
where
    I: From<usize> + Copy + core::cmp::Eq + core::hash::Hash,
{
    pub fn new() -> Self {
        IdTable {
            map: HashMap::new(),
            available_ids: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: T) -> I {
        let element_id = {
            if let Some(id) = self.available_ids.pop() {
                id
            } else {
                self.map.len().into()
            }
        };
        self.map.insert(element_id, element);
        element_id
    }

    pub fn try_get_element(&self, element_id: I) -> Option<&T> {
        self.map.get(&element_id)
    }

    pub fn remove_element(&mut self, element_id: I) {
        self.map.remove(&element_id);
        self.available_ids.push(element_id);
    }
}
