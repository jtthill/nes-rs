/// Tests the bit in the given 8-bit `data` at the position given
/// by `pos`. Returns a `bool` where true is 1, and false is 0. 
/// Returns false on a `pos` greater than 7.
pub fn test_bit_u8(data: &u8, pos: u8) -> bool {
    if pos > 7 {
        return false;
    }
    let mask = 1 << pos;
    let val = data & mask;
    val == 1
}

/// Sets the bit in the given 8-bit `data` at the position given
/// by `pos`. Returns with no changes on a `pos` greater than 7.
pub fn set_bit_u8(data: &mut u8, pos: u8) {
    if pos > 7 {
        return;
    }
    let mask = 1 << pos;
    *data |= mask;
}

/// Clears the bit in the given 8-bit `data` at the position given
/// by `pos`. Returns with no changes on a `pos` greater than 7.
pub fn clear_bit_u8(data: &mut u8, pos: u8) {
    if pos > 7 {
        return;
    }
    let mask = !(1 << pos);
    *data &= mask;
}
