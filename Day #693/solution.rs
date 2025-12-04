// Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
// Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
// Time O(1), Space O(1).
fn swap_bits(n: u8) -> u8 {
    ((n & 0xAA) >> 1) | ((n & 0x55) << 1)
}

fn main() {
    println!("{:08b}", swap_bits(0b10101010)); // 01010101
    println!("{:08b}", swap_bits(0b11100010)); // 11010001
}
