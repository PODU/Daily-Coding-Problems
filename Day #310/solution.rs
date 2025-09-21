// Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
fn count_bits(n: i64) -> i64 {
    let mut total = 0i64;
    let mut i = 0;
    while (1i64 << i) <= n {
        let blk = 1i64 << (i + 1);
        let full = (n + 1) / blk * (1i64 << i);
        let rem = ((n + 1) % blk - (1i64 << i)).max(0);
        total += full + rem;
        i += 1;
    }
    total
}

fn main() {
    println!("{}", count_bits(5)); // 7
}
