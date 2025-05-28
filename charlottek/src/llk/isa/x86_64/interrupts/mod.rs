pub mod exceptions;
pub mod idt;

use idt::*;
use spin::Mutex;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());
