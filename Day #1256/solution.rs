// Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
// Time O(n), Space O(1).
fn max_sub(a: &[i64]) -> i64 {
    let mut cur = 0i64;
    let mut best = 0i64;
    for &x in a {
        cur += x;
        if cur < 0 {
            cur = 0;
        }
        if cur > best {
            best = cur;
        }
    }
    best
}

fn main() {
    println!("{}", max_sub(&[34, -50, 42, 14, -5, 86]));
    println!("{}", max_sub(&[-5, -1, -8, -9]));
}
