// Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
// For each bit i: full cycles contribute 2^i ones each, plus remainder.
fn count_set_bits(n: i64) -> i64 {
    let mut total: i64 = 0;
    let mut i = 0;
    while (1i64 << i) <= n {
        let block = 1i64 << (i + 1);
        let mut ones = (n + 1) / block * (1i64 << i);
        let rem = (n + 1) % block - (1i64 << i);
        if rem > 0 {
            ones += rem;
        }
        total += ones;
        i += 1;
    }
    total
}

fn main() {
    println!("N=5  -> {}", count_set_bits(5));   // 7
    println!("N=16 -> {}", count_set_bits(16));  // 33
}
