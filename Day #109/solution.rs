// Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
fn swap_bits(x: u8) -> u8 {
    ((x & 0xAA) >> 1) | ((x & 0x55) << 1)
}

fn main() {
    println!("{:08b}", swap_bits(0b10101010)); // 01010101
    println!("{:08b}", swap_bits(0b11100010)); // 11010001
}
