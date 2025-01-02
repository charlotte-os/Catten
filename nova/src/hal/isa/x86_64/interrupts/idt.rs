use core::arch::asm;
use core::ptr::addr_of;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Idt {
    pub gates: [InterruptGate; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            gates: [InterruptGate::new(); 256],
        }
    }
    pub fn set_gate(
        &mut self,
        index: usize,
        isr_ptr: unsafe extern "C" fn(),
        segment_selector: u16,
        is_trap: bool,
        is_present: bool,
    ) {
        let gate = &mut self.gates[index];
        let isr_addr = isr_ptr as u64;

        gate.addr0 = u16::try_from(isr_addr & 0xFFFF).unwrap();
        gate.segment_selector = segment_selector;
        gate.reserved_ist_index = 0u8; // the IST is not used
        gate.flags = if is_trap { 0b1111u8 } else { 0b1110u8 }; //gate type
                                                                //reserved bit
        gate.flags &= !(0b1u8 << 4);
        //privilege ring required to use gate
        gate.flags &= !(0b11u8 << 5);
        //present bit
        if is_present {
            gate.flags |= 0b1u8 << 7;
        } else {
            gate.flags &= !(0b1u8 << 7);
        }
        gate.addr1 = ((isr_addr & (0xFFFF << 16)) >> 16) as u16;
        gate.addr2 = ((isr_addr & (0xFFFFFFFF << 32)) >> 32) as u32;
        gate.reserved = 0u32;
    }
    #[allow(unused)]
    pub fn set_present(&mut self, index: usize) {
        if index < 256 {
            self.gates[index].flags |= 0b1u8 << 7;
        }
    }
    #[allow(unused)]
    pub fn clear_present(&mut self, index: usize) {
        if index < 256 {
            self.gates[index].flags &= !(0b1u8 << 7);
        }
    }
    pub fn load(&self) {
        let idtr = Idtr::new(128u16 * 256u16 - 1u16, addr_of!(*self) as u64);
        unsafe {
            asm_load_idt(&idtr);
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct InterruptGate {
    addr0: u16,
    segment_selector: u16,
    reserved_ist_index: u8,
    flags: u8,
    addr1: u16,
    addr2: u32,
    reserved: u32,
}

impl InterruptGate {
    const fn new() -> Self {
        InterruptGate {
            addr0: 0u16,
            segment_selector: 0u16,
            reserved_ist_index: 0u8,
            flags: 0u8,
            addr1: 0u16,
            addr2: 0u32,
            reserved: 0u32,
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    size: u16,
    base: u64,
}

impl Idtr {
    fn new(size: u16, base: u64) -> Self {
        Idtr { size, base }
    }
}

unsafe fn asm_load_idt(idtr: &Idtr) {
    asm!("\
        lidt [{}]
    ", in(reg) idtr);
}
