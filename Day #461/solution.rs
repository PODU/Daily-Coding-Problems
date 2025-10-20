// Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
// Multiplicative binomial. Time O(min(N,M)), Space O(1).
fn count_paths(n: i64, m: i64) -> i64 {
    let a = (n - 1) + (m - 1);
    let b = (n - 1).min(m - 1);
    let mut res: i64 = 1;
    for i in 1..=b {
        res = res * (a - b + i) / i;
    }
    res
}

fn main() {
    println!("{}", count_paths(2, 2)); // 2
    println!("{}", count_paths(5, 5)); // 70
}
