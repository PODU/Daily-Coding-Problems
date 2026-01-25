// Day 959: total number of set bits over all integers in [1, N].
// Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).

fn count_set_bits(n: i64) -> i64 {
    let mut total = 0i64;
    let mut i = 0;
    while (1i64 << i) <= n {
        let cycle = 1i64 << (i + 1);
        let half = cycle >> 1;
        total += (n + 1) / cycle * half;
        let rem = (n + 1) % cycle;
        if rem - half > 0 {
            total += rem - half;
        }
        i += 1;
    }
    total
}

fn main() {
    println!("{}", count_set_bits(5)); // 7
}
