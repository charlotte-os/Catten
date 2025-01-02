pub mod exceptions;
pub mod idt;

use core::mem::MaybeUninit;

use spin::Mutex;

use idt::*;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());