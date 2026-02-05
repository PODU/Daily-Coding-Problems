// Day 1024: Reverse all 32 bits of a 32-bit integer.
// Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.
fn reverse_bits(mut n: u32) -> u32 {
    n = (n >> 16) | (n << 16);
    n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8);
    n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4);
    n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1);
    n
}

fn grouped(n: u32) -> String {
    let bits = format!("{:032b}", n);
    let mut parts: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < 32 {
        parts.push(&bits[i..i + 4]);
        i += 4;
    }
    parts.join(" ")
}

fn main() {
    let x: u32 = 0xF0F0F0F0;
    println!("{}", grouped(reverse_bits(x)));
}
