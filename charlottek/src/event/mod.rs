pub trait Event {
    fn register_observer(&mut self, observer: &dyn Observer);
}

pub trait Observer {
    fn notify(&mut self);
}

pub struct Completion<F: Fn() + Send + Sync + 'static> {
    completed: bool,
    callback:  Option<F>,
}

impl<F> Completion<F>
where
    F: Fn() + Send + Sync + 'static,
{
    pub fn new(callback: Option<F>) -> Self {
        Completion {
            completed: false,
            callback,
        }
    }

    pub fn poll(&self) -> bool {
        self.completed
    }

    pub fn register_callback(&mut self, callback: F) {
        self.callback.replace(callback);
    }
}

impl<F> Observer for Completion<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn notify(&mut self) {
        self.completed = true;
        if let Some(ref cb) = self.callback {
            cb();
        }
    }
}

pub struct Sentinel<F: Fn() + Send + Sync + 'static> {
    times_notified: u64,
    callback: Option<F>,
}

impl<F> Sentinel<F>
where
    F: Fn() + Send + Sync + 'static,
{
    pub fn new(callback: Option<F>) -> Self {
        Sentinel {
            times_notified: 0,
            callback,
        }
    }

    pub fn get_times_notified(&self) -> u64 {
        self.times_notified
    }

    pub fn register_callback(&mut self, callback: F) {
        self.callback.replace(callback);
    }
}

impl<F> Observer for Sentinel<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn notify(&mut self) {
        self.times_notified += 1;
        if let Some(ref cb) = self.callback {
            cb();
        }
    }
}
