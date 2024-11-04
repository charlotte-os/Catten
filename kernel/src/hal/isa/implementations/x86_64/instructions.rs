use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al", 
            in("dx") port, in("al") value
        );
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx", 
            out("al") value, in("dx") port
        );
    }
    value
}