pub trait InstructionInterface {
    unsafe fn inb(port: u16) -> u8;
    unsafe fn outb(port: u16, value: u8);
    fn halt() -> !;
}