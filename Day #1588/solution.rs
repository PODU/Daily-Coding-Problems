// Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
// Time: O(N*M); Space: O(N*M).
fn count_paths(grid: &[Vec<i32>]) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    if grid[0][0] == 1 || grid[n - 1][m - 1] == 1 {
        return 0;
    }
    let mut dp = vec![vec![0u64; m]; n];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
            } else if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else {
                let up = if i > 0 { dp[i - 1][j] } else { 0 };
                let left = if j > 0 { dp[i][j - 1] } else { 0 };
                dp[i][j] = up + left;
            }
        }
    }
    dp[n - 1][m - 1]
}

fn main() {
    let grid = vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 0, 0]];
    println!("{}", count_paths(&grid));
}
