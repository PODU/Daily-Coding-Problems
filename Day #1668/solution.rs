// Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
fn digit_sum(mut x: u64) -> u64 { let mut s = 0; while x > 0 { s += x % 10; x /= 10; } s }
fn nth_perfect(n: u32) -> u64 { let (mut x, mut c) = (0u64, 0u32); while c < n { x += 1; if digit_sum(x) == 10 { c += 1; } } x }
fn main() { println!("{}", nth_perfect(1)); println!("{}", nth_perfect(2)); }
