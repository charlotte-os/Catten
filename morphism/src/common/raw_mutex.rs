use lock_api::GuardSend;

pub struct RawMutex {
    inner: spin::Mutex<()>,
}
impl RawMutex {
    pub const fn new() -> Self {
        RawMutex {
            inner: spin::Mutex::new(()),
        }
    }
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = GuardSend;

    const INIT: Self = Self::new();

    fn lock(&self) {
        self.inner.lock();
    }

    fn try_lock(&self) -> bool {
        self.inner.try_lock().is_some()
    }

    unsafe fn unlock(&self) {
        self.inner.unlock()
    }
}
