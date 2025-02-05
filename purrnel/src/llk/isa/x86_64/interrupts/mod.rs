pub mod exceptions;
pub mod idt;

use core::mem::MaybeUninit;

use idt::*;
use spin::Mutex;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());
