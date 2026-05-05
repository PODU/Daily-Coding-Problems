// Day 1473: Count occurrences of X in an N x N multiplication table.
// For each row i, X appears iff i divides X and X/i is within [1, N].
// Time O(N), Space O(1).

fn count_x(n: i64, x: i64) -> i64 {
    let mut count = 0;
    for i in 1..=n {
        if x % i == 0 && x / i >= 1 && x / i <= n {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_x(6, 12)); // 4
}
