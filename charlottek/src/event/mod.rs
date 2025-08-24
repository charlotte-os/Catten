use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, Ordering};

use spin::mutex::Mutex;

pub trait Event {
    fn register_observer(&mut self, observer: &dyn Observer);
}

pub trait Observer {
    fn notify(&self);
}

pub struct Completion {
    completed: AtomicBool,
    callback:  Mutex<Option<fn()>>,
}

impl Completion {
    pub fn new() -> Self {
        Completion {
            completed: AtomicBool::new(false),
            callback:  Mutex::new(None),
        }
    }

    pub fn poll(&self) -> bool {
        self.completed.load(Ordering::Acquire)
    }

    pub fn register_callback(&mut self, callback: fn()) {
        self.callback.lock().replace(callback);
    }
}

impl Observer for Completion {
    fn notify(&self) {
        if let Ok(_) =
            self.completed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        {
            if let Some(on_complete) = &self.callback.lock().as_ref() {
                on_complete()
            }
        }
    }
}
