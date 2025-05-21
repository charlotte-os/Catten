use lock_api::GuardSend;
use spin::Mutex;

pub struct RawMutex {
    raw: Mutex<bool>,
}
impl RawMutex {
    pub const fn new() -> Self {
        RawMutex { raw: Mutex::new(false) }
    }
}

unsafe impl lock_api::RawMutex for RawMutex {
    type GuardMarker = GuardSend;

    const INIT: Self = Self::new();

    fn lock(&self) {
        *self.raw.lock() = true;
    }

    fn try_lock(&self) -> bool {
        match *self.raw.lock() {
            true => false,
            false => {
                *(self.raw.lock()) = true;
                true
            }
        }
    }

    unsafe fn unlock(&self) {
        *self.raw.lock() = false;
    }
}
