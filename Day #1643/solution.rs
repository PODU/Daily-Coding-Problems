// Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
// the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).

fn reverse_bits(mut n: u32) -> u32 {
    let mut result: u32 = 0;
    for _ in 0..32 {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    result
}

fn group_nibbles(n: u32) -> String {
    let bits = format!("{:032b}", n);
    (0..32)
        .step_by(4)
        .map(|i| &bits[i..i + 4])
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let value: u32 = 0xF0F0F0F0;
    println!("{}", group_nibbles(reverse_bits(value)));
}
