use core::arch::asm;

struct X86_64InstructionInterface;

impl super::interface::InstructionInterface for X86_64InstructionInterface {
    unsafe fn outb(port: u16, value: u8) {
        unsafe {
            asm!(
                "out dx, al",
                in("dx") port, in("al") value
            );
        }
    }
    unsafe fn inb(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                out("al") value, in("dx") port
            );
        }
        value
    }
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }
}
