//! # Input/Output Traits and Blanket Implementations

pub trait Read {
    // Required
    fn read(&mut self, buf: &mut [u8]) -> usize;
    // Provided
    #[inline]
    fn read_byte(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.read(&mut buf);
        buf[0]
    }
    #[inline(always)]
    fn read_line(&mut self, buf: &mut [u8]) -> usize {
        self.read_until(buf, b'\n')
    }
    #[inline]
    fn read_until(&mut self, buf: &mut [u8], delim: u8) -> usize {
        let n_bytes = buf.len();
        for i in 0..n_bytes {
            let c = self.read_byte();
            if c == delim {
                return i;
            }
            buf[i] = c;
        }
        n_bytes
    }
    #[inline]
    fn read_n(&mut self, buf: &mut [u8], n: usize) -> usize {
        let n_bytes = core::cmp::max(n, buf.len());
        for i in 0..n_bytes {
            buf[i] = self.read_byte();
        }
        n_bytes
    }
}
