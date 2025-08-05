use alloc::vec::Vec;

use super::PAddr;

pub struct FrameSet {
    frames: Vec<PAddr>,
    map_count: usize,
}

impl FrameSet {
    pub fn new(frames: Vec<PAddr>) -> Self {
        FrameSet {
            frames,
            map_count: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.frames.len()
    }

    pub fn as_slice(&self) -> &[PAddr] {
        &self.frames
    }
}
#[repr(u8)]
pub enum FrameSetPermissions {
    MapRead = 1 << 0,
    MapWrite = 1 << 1,
    MapExecute = 1 << 2,
}
