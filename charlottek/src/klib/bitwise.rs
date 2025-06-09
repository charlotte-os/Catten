pub fn assign_bits_at(value: &mut u64, index: u8, len: u8, mut bits: u64) {
    for i in index..(index + len) {
        //clear the area before setting the bits
        *value &= !(1 << i);
        *value |= (bits & 1) << i;
        bits >>= 1;
    }
}
