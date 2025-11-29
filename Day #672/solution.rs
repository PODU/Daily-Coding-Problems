// Day 672: Maximum-weight top-to-bottom triangle path. Bottom-up DP folding rows.
// Time O(n^2) cells, Space O(n).
fn max_path(t: &[Vec<i32>]) -> i32 {
    let mut dp = t[t.len() - 1].clone();
    for i in (0..t.len() - 1).rev() {
        for j in 0..=i {
            dp[j] = t[i][j] + dp[j].max(dp[j + 1]);
        }
    }
    dp[0]
}

fn main() {
    let t = vec![vec![1], vec![2, 3], vec![1, 5, 1]];
    println!("{}", max_path(&t)); // 9
}
