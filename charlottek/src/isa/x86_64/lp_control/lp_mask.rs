use alloc::vec::Vec;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering::AcqRel;

use super::thread::Error as ThreadError;
use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};
use crate::klib::constants::BITS_PER_BYTE;
use crate::multiprocessing::get_lp_count;

pub struct LpMask {
    mask: Vec<AtomicU8>,
}

impl LpMask {
    pub fn new() -> Self {
        let mut capacity = get_lp_count() / BITS_PER_BYTE as u32;
        if get_lp_count() % BITS_PER_BYTE as u32 > 0 {
            capacity += 1;
        }
        LpMask {
            mask: Vec::with_capacity(capacity as usize),
        }
    }

    pub fn set(&mut self, lp_id: <LpControl as LpControlIfce>::LpId) -> Result<(), ThreadError> {
        if lp_id > get_lp_count() {
            return Err(ThreadError::InvalidLp);
        }
        let (byte_idx, bit_idx) = (lp_id / BITS_PER_BYTE as u32, lp_id % BITS_PER_BYTE as u32);
        if (self.mask.len() as u32) < byte_idx {
            return Err(ThreadError::InvalidLp);
        }

        self.mask[byte_idx as usize].update(AcqRel, AcqRel, |byte| byte | 1 << bit_idx);
        Ok(())
    }
}
