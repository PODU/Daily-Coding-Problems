// Count occurrences of X in an N x N multiplication table.
// For each row i (1..N), X appears iff i divides X and X/i is in [1,N]. O(N) time, O(1) space.
fn count_x(n: i64, x: i64) -> i64 {
    let mut cnt = 0;
    for i in 1..=n {
        if x % i == 0 {
            let q = x / i;
            if q >= 1 && q <= n {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    println!("{}", count_x(6, 12)); // 4
}
