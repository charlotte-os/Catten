use core::arch::asm;

pub struct InstructionInterface;

impl super::interface::InstructionInterface for InstructionInterface {
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
