// Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
// One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
fn swap_bits(n: u8) -> u8 {
    ((n & 0xAA) >> 1) | ((n & 0x55) << 1)
}

fn main() {
    for s in ["10101010", "11100010"] {
        let n = u8::from_str_radix(s, 2).unwrap();
        println!("{:08b}", swap_bits(n));
    }
}
