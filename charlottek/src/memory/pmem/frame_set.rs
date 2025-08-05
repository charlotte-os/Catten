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

    pub fn iter(&self) -> AddrIter {
        AddrIter {
            index: 0,
            raw_frame_set: &self.frames,
        }
    }
}

// 'fs is the lifetime of the FrameSet being iterated over
pub struct AddrIter<'fs> {
    index: usize,
    raw_frame_set: &'fs [PAddr],
}
