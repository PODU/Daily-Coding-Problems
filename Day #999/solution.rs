// Day 999: Count occurrences of X in an N x N multiplication table.
// X appears at (i, j) iff i divides X and X/i <= N, for i in 1..N. O(N) time.
fn count_x(n: i32, x: i32) -> i32 {
    (1..=n).filter(|&i| x % i == 0 && x / i <= n).count() as i32
}

fn main() {
    println!("{}", count_x(6, 12)); // 4
}
