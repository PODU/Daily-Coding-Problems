// Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, shift input right.
// Time O(32)=O(1), space O(1). Uses u32 for unsigned 32-bit arithmetic.

fn reverse_bits(mut x: u32) -> u32 {
    let mut result: u32 = 0;
    for _ in 0..32 {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

fn main() {
    let out = reverse_bits(0xF0F0F0F0u32);
    let bits = format!("{:032b}", out);
    let groups: Vec<String> = (0..32).step_by(4).map(|i| bits[i..i + 4].to_string()).collect();
    println!("{}", groups.join(" "));
}
