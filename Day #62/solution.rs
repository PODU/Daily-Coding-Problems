// Count lattice paths in N x M grid via combinatorics C(n+m-2, n-1).
// Time O(min(n,m)) multiplicative, Space O(1).
fn paths(n: i64, m: i64) -> i64 {
    let down = n - 1;
    let right = m - 1;
    let k = down.min(right);
    let total = down + right;
    let mut res: i64 = 1;
    for i in 1..=k {
        res = res * (total - k + i) / i;
    }
    res
}

fn main() {
    println!("2 by 2 matrix -> {}", paths(2, 2));
    println!("5 by 5 matrix -> {}", paths(5, 5));
}
