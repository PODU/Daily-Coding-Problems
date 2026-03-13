// Reverse all 32 bits by shifting LSB of input into LSB-first of result.
// Time: O(1) (32 steps); Space: O(1).
fn reverse_bits(mut x: u32) -> u32 {
    let mut r: u32 = 0;
    for _ in 0..32 {
        r = (r << 1) | (x & 1);
        x >>= 1;
    }
    r
}

fn to_grouped(x: u32) -> String {
    let mut s = String::new();
    for i in (0..32).rev() {
        s.push(if (x >> i) & 1 == 1 { '1' } else { '0' });
        if i % 4 == 0 && i != 0 {
            s.push(' ');
        }
    }
    s
}

fn main() {
    let input: u32 = 0xF0F0F0F0;
    println!("Input:  {}", to_grouped(input));
    println!("{}", to_grouped(reverse_bits(input)));
}
