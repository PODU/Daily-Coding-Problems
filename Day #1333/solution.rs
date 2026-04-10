// Day 1333: Count right/down paths from top-left to bottom-right avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. O(N*M) time, O(M) space.

fn count_paths(g: &[Vec<i32>]) -> i64 {
    let n = g.len();
    let m = g[0].len();
    let mut dp = vec![0i64; m];
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == 1 {
                dp[j] = 0;
            } else if i == 0 && j == 0 {
                dp[j] = 1;
            } else {
                let up = if i > 0 { dp[j] } else { 0 };
                let left = if j > 0 { dp[j - 1] } else { 0 };
                dp[j] = up + left;
            }
        }
    }
    dp[m - 1]
}

fn main() {
    let g = vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 0, 0]];
    println!("{}", count_paths(&g)); // 2
}
