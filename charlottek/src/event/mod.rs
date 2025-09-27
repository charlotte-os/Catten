pub trait Event {
    fn register_observer(&mut self, observer: &dyn Observer);
}

pub trait Observer {
    fn notify(&mut self);
}

pub struct Completion {
    completed: bool,
    callback:  Option<fn()>,
}

impl Completion {
    pub fn new(callback: Option<fn()>) -> Self {
        Completion {
            completed: false,
            callback,
        }
    }

    pub fn poll(&self) -> bool {
        self.completed
    }

    pub fn register_callback(&mut self, callback: fn()) {
        self.callback.replace(callback);
    }
}

impl Observer for Completion {
    fn notify(&mut self) {
        self.completed = true;
        if let Some(cb) = self.callback {
            cb();
        }
    }
}

pub struct Sentinel {
    times_notified: u64,
    callback: Option<fn()>,
}

impl Sentinel {
    pub fn new(callback: Option<fn()>) -> Self {
        Sentinel {
            times_notified: 0,
            callback,
        }
    }

    pub fn get_times_notified(&self) -> u64 {
        self.times_notified
    }

    pub fn register_callback(&mut self, callback: fn()) {
        self.callback.replace(callback);
    }
}

impl Observer for Sentinel {
    fn notify(&mut self) {
        self.times_notified += 1;
        if let Some(cb) = self.callback {
            cb();
        }
    }
}
