pub fn test_bit_u8(data: &u8, pos: u8) -> bool {
    if pos > 7 {
        return false;
    }
    let mask = 1 << pos;
    let val = data & mask;
    val == 1
}
