// Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
// into the result. Time O(32), Space O(1).

fn reverse_bits(mut n: u32) -> u32 {
    let mut res: u32 = 0;
    for _ in 0..32 {
        res = (res << 1) | (n & 1);
        n >>= 1;
    }
    res
}

fn to_groups(n: u32) -> String {
    let bits = format!("{:032b}", n);
    bits.as_bytes()
        .chunks(4)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let n: u32 = 0xF0F0F0F0; // 1111 0000 ... repeated
    println!("{}", to_groups(reverse_bits(n)));
}
