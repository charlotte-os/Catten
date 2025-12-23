use alloc::vec::Vec;

pub struct VecQueue<T: Clone> {
    vec:  Vec<T>,
    head: usize,
    tail: usize,
}

impl<T: Clone> VecQueue<T> {
    pub fn new() -> Self {
        VecQueue {
            vec:  Vec::<T>::new(),
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.head == self.tail && self.tail == 0usize {
            self.vec.extend_reserve(self.vec.len() / 2);
        }
        self.vec[self.tail] = elem;
        self.tail = (self.tail + 1) % self.vec.len();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail && self.tail != 0 {
            None
        } else {
            let old_head = self.head;
            self.head = (self.head + 1) % self.vec.len();
            Some(self.vec[old_head].clone())
        }
    }
}
