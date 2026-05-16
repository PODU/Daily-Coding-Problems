// Count cells equal to X in an N x N multiplication table (cell(i,j)=i*j).
// For each row i, X is at column X/i iff i divides X and 1<=X/i<=N. O(N) time, O(1) space.
fn count_cells(n: i64, x: i64) -> i64 {
    let mut count = 0;
    let mut i = 1;
    while i <= n {
        if x % i == 0 {
            let j = x / i;
            if j >= 1 && j <= n {
                count += 1;
            }
        }
        i += 1;
    }
    count
}

fn main() {
    println!("{}", count_cells(6, 12)); // expected 4
}
