use core::arch::asm;
use core::mem::size_of;
use core::ptr;

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
struct SegmentDescriptor {
    limit0: u16,
    base0: u16,
    base1: u8,
    access_byte: u8,
    limit1_flags: u8,
    base2: u8,
}

impl SegmentDescriptor {
    fn new() -> Self {
        SegmentDescriptor {
            limit0: 0,
            base0: 0,
            base1: 0,
            access_byte: 0,
            limit1_flags: 0,
            base2: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(1))]
struct TssDescriptor {
    low:  u64, // Low 64 bits of the descriptor
    high: u64, // High 64 bits of the descriptor
}

impl TssDescriptor {
    fn new() -> Self {
        TssDescriptor {
            low:  0,
            high: 0,
        }
    }
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Gdt {
    segment_descs: [SegmentDescriptor; 5],
    tss_desc: TssDescriptor,
}

impl Gdt {
    pub fn new(tss: &Tss) -> Self {
        let mut gdt = Gdt {
            segment_descs: [SegmentDescriptor::new(); 5],
            tss_desc: TssDescriptor::new(),
        };

        //Null descriptor
        gdt.set_segment_desc(0, 0, 0, 0, 0);
        //Kernel Mode Code Segment
        gdt.set_segment_desc(1, 0, 0xfffff, 0x9a, 0xa);
        //Kernel Mode Data Segment
        gdt.set_segment_desc(2, 0, 0xfffff, 0x92, 0xc);
        //User Mode Code Segment
        gdt.set_segment_desc(3, 0, 0xfffff, 0xfa, 0xa);
        //User Mode Data Segment
        gdt.set_segment_desc(4, 0, 0xfffff, 0xf2, 0xc);
        //Task State Segment
        gdt.set_tss_desc(ptr::addr_of!(*tss) as u64, size_of::<Tss>() as u32);

        gdt
    }

    fn set_tss_desc(&mut self, base: u64, limit: u32) {
        let low = ((limit as u64) & 0xFFFF)
            | ((base & 0xFFFFFF) << 16)
            | ((0x89u64) << 40) // Type for TSS
            | ((limit as u64 & 0xF0000) << 32)
            | ((base & 0xFF000000) << 32);
        let high = (base >> 32) & 0xffffffff;

        self.tss_desc.low = low;
        self.tss_desc.high = high;
    }

    fn set_segment_desc(
        &mut self,
        index: usize,
        base: u32,
        limit: u32,
        access_byte: u8,
        flags: u8,
    ) {
        let dest_sd = &mut (self.segment_descs[index]);

        dest_sd.limit0 = (limit & 0xffff) as u16;
        dest_sd.limit1_flags = ((limit & 0xff0000) >> 16) as u8; // Only the lower 4 bits of this field encodes limit bits

        dest_sd.base0 = (base & 0xffff) as u16;
        dest_sd.base1 = ((base & 0xff0000) >> 16) as u8;
        dest_sd.base2 = ((base & 0xff000000) >> 24) as u8;

        dest_sd.access_byte = access_byte;

        dest_sd.limit1_flags |= (flags & 0xff) << 4; // The upper 4 bits
        // of this field
        // encodes flags
    }

    pub fn load(&self) {
        GdtRegister::new(self).load();
        /* load the tss */
        unsafe {
            asm!("ltr [rip + TSS_SELECTOR]");
        }
    }

    pub fn reload_segment_regs() {
        unsafe {
            asm!(
                "movzx rax, word ptr [rip + KERNEL_CODE_SELECTOR]",
                "push rax",
                "lea rax, [rip + reload_cs]",
                "push rax",
                "retfq",
                "reload_cs:",
                "mov ax, [rip + KERNEL_DATA_SELECTOR]",
                "mov ds, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                "mov ss, ax",
                options(nomem, nostack, preserves_flags),
            );
        }
    }
}

#[repr(C, packed(1))]
struct GdtRegister {
    limit: u16,
    base:  *const Gdt,
}

impl GdtRegister {
    fn new(gdt: &Gdt) -> Self {
        GdtRegister {
            limit: (size_of::<Gdt>() - 1) as u16,
            base:  gdt,
        }
    }

    fn load(&self) {
        unsafe {
            asm!(
                "lgdt [{}]",
                in(reg) self,
            );
        }
    }
}

type SegmentSelector = u16;
/* Segment Selector Format:
[15:3] Index of the segment descriptor in the GDT or LDT
[2] Table Indicator (0 for GDT, 1 for LDT)
[1:0] Requested Privilege Level (RPL) (0 for kernel, 3 for user)
*/

const SEGMENT_SELECTOR_INDEX_SHIFT: u16 = 3; // Shift for segment selector index
const SEGMENT_SELECTOR_KERNEL_PRIVILEGE_LEVEL: u16 = 0b00; // Kernel privilege level
const SEGMENT_SELECTOR_USER_PRIVILEGE_LEVEL: u16 = 0b11; // User privilege level

const fn make_segment_selector(index: u16, is_user_mode: bool) -> SegmentSelector {
    (index << SEGMENT_SELECTOR_INDEX_SHIFT)
        | if is_user_mode {
            SEGMENT_SELECTOR_USER_PRIVILEGE_LEVEL
        } else {
            SEGMENT_SELECTOR_KERNEL_PRIVILEGE_LEVEL
        }
}
#[allow(unused)]
#[unsafe(no_mangle)]
pub static NULL_SELECTOR: SegmentSelector = make_segment_selector(0, false);
#[allow(unused)]
#[unsafe(no_mangle)]
pub static KERNEL_CODE_SELECTOR: SegmentSelector = make_segment_selector(1, false);
#[allow(unused)]
#[unsafe(no_mangle)]
pub static KERNEL_DATA_SELECTOR: SegmentSelector = make_segment_selector(2, false);
#[allow(unused)]
#[unsafe(no_mangle)]
pub static USER_CODE_SELECTOR: SegmentSelector = make_segment_selector(3, true);
#[allow(unused)]
#[unsafe(no_mangle)]
pub static USER_DATA_SELECTOR: SegmentSelector = make_segment_selector(4, true);
#[allow(unused)]
#[unsafe(no_mangle)]
pub static TSS_SELECTOR: SegmentSelector = make_segment_selector(5, false);

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Tss {
    res0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    res1: u64,
    ist1: u64,
    ist2: u64,
    ist3: u64,
    ist4: u64,
    ist5: u64,
    ist6: u64,
    ist7: u64,
    res2: u64,
    iopb: u16,
    res3: u16,
}

impl Tss {
    pub fn new(rsp0: u64) -> Self {
        Tss {
            res0: 0,
            rsp0: rsp0,
            rsp1: 0,
            rsp2: 0,
            res1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            res2: 0,
            iopb: size_of::<Tss>() as u16,
            res3: 0,
        }
    }
}
