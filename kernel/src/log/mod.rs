#[macro_export]
macro_rules! log {
    ($text:expr $(, $arg:tt)*) => ({
        use core::fmt::Write;
        use crate::hal::drivers::rs232::LOG_PORT;
        let _ = write!(LOG_PORT.lock(), $text $(, $arg)*);
    })
}
#[macro_export]
macro_rules! logln {
    ($text:expr $(, $arg:tt)*) => ({
        use core::fmt::Write;
        use crate::hal::drivers::rs232::LOG_PORT;
        let _ = writeln!(LOG_PORT.lock(), $text $(, $arg)*);
    })
}