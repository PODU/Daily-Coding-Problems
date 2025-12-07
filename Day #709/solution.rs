// Day 709: Count paths top-left to bottom-right moving right/down, avoiding walls.
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. Time O(N*M), space O(M).
fn count_paths(g: &[Vec<i32>]) -> u64 {
    let n = g.len();
    let m = g[0].len();
    let mut dp = vec![0u64; m];
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == 1 {
                dp[j] = 0;
            } else if i == 0 && j == 0 {
                dp[j] = 1;
            } else {
                let left = if j > 0 { dp[j - 1] } else { 0 };
                dp[j] += left;
            }
        }
    }
    dp[m - 1]
}

fn main() {
    let grid = vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 0, 0]];
    println!("{}", count_paths(&grid));
}
