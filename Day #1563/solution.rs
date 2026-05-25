// Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).
fn count_paths(n: i64, m: i64) -> i64 {
    let total = n + m - 2;
    let k = std::cmp::min(n - 1, m - 1);
    let mut res: i64 = 1;
    for i in 1..=k {
        res = res * (total - k + i) / i;
    }
    res
}

fn main() {
    println!("{}", count_paths(2, 2));
}
