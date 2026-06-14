// Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
fn max_path(t: &[Vec<i32>]) -> i32 {
    let n = t.len();
    let mut dp = t[n - 1].clone();
    for i in (0..n - 1).rev() {
        for j in 0..=i {
            dp[j] = t[i][j] + dp[j].max(dp[j + 1]);
        }
    }
    dp[0]
}

fn main() {
    let t = vec![vec![1], vec![2, 3], vec![1, 5, 1]];
    println!("{}", max_path(&t));
}
