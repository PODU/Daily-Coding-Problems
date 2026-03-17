// Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.
fn count_set_bits(n: i64) -> i64 {
    let mut total = 0i64;
    let mut p = 2i64;
    while p <= 2 * n {
        let full = ((n + 1) / p) * (p / 2);
        let rem = std::cmp::max(0, (n + 1) % p - p / 2);
        total += full + rem;
        p <<= 1;
    }
    total
}

fn main() {
    println!("{}", count_set_bits(5));
}
