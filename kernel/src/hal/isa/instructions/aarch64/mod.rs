use core::arch::asm;

struct InstructionInterface;

impl super::interface::InstructionInterface for InstructionInterface {
    #[inline(always)]
    unsafe fn outb(port: u16, value: u8) {
        unsafe {
            // Aarch64 doesn't support IO Ports so this is a no-op
            asm!(
                "nop"
            );
        }
    }
    #[inline(always)]
    unsafe fn inb(port: u16) -> u8 {
        0
    }
    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("wfi");
        }
        loop {}
    }
}