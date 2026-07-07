// Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
// Time: O(1), Space: O(1).
fn swap_bits(bin: &str) -> String {
    let n = u32::from_str_radix(bin, 2).unwrap();
    let r = (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
    format!("{:08b}", r)
}

fn main() {
    println!("{}", swap_bits("10101010"));
    println!("{}", swap_bits("11100010"));
}
