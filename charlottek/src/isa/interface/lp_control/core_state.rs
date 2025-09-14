pub trait CoreStateIfce {
    extern "C" fn save(&mut self);
    extern "C" fn load(&self);
}
