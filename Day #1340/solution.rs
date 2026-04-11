// Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
// Overflow-safe multiplicative loop. Time O(N+M), Space O(1).

fn count_paths(n: i64, m: i64) -> u64 {
    let total = n + m - 2;
    let k = (n - 1).min(m - 1);
    let mut res: u64 = 1;
    for i in 1..=k {
        res = res * (total - k + i) as u64 / i as u64;
    }
    res
}

fn main() {
    println!("2 by 2 -> {}", count_paths(2, 2));
    println!("5 by 5 -> {}", count_paths(5, 5));
}
