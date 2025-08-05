// Count occurrences of X in N×N table: for each row i (1..N), X appears iff i|X and X/i in [1..N].
// Time O(N), Space O(1).
fn count_x(n: i64, x: i64) -> i64 {
    let mut cnt = 0;
    for i in 1..=n {
        if x % i == 0 && x / i >= 1 && x / i <= n {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    println!("{}", count_x(6, 12));
}
