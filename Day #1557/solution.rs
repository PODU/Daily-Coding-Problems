// Grid DP: max coins from top-left to bottom-right moving right/down only.
// dp[j] = grid + max(top, left). Time O(m*n), Space O(n).
fn main() {
    let grid = vec![
        vec![0, 3, 1, 1],
        vec![2, 0, 0, 4],
        vec![1, 5, 3, 1],
    ];
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            let best = if i == 0 && j == 0 {
                0
            } else if i == 0 {
                dp[j - 1]
            } else if j == 0 {
                dp[j]
            } else {
                dp[j].max(dp[j - 1])
            };
            dp[j] = grid[i][j] + best;
        }
    }
    println!("{}", dp[n - 1]);
}
