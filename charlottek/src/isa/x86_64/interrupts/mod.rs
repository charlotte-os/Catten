pub mod exceptions;
pub mod idt;
pub mod ipi;

use idt::*;
use spin::Mutex;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());
