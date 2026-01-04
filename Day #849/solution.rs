// Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
// Memoized chain lengths via a Vec. ~O(limit) amortized.
fn steps(mut n: u64) -> u32 {
    let mut c = 0;
    while n != 1 {
        n = if n & 1 == 1 { 3 * n + 1 } else { n / 2 };
        c += 1;
    }
    c
}

fn main() {
    println!("27 reaches 1 in {} steps", steps(27)); // 111

    const LIMIT: usize = 1_000_000;
    let mut cache = vec![0i32; LIMIT + 1];
    cache[1] = 1;
    let (mut best_n, mut best_len) = (1usize, 1i32);
    for i in 2..=LIMIT {
        let mut n = i as u64;
        let mut len = 0i32;
        while n >= i as u64 || cache[n as usize] == 0 {
            n = if n & 1 == 1 { 3 * n + 1 } else { n / 2 };
            len += 1;
            if n == 1 {
                break;
            }
        }
        let total = len + if (n as usize) <= LIMIT { cache[n as usize] } else { 1 };
        cache[i] = total;
        if total > best_len {
            best_len = total;
            best_n = i;
        }
    }
    println!(
        "Longest chain for n <= 1000000: n = {} (length {})",
        best_n, best_len
    ); // 837799
}
