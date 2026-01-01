// Day 838: Max coins moving only right/down through a grid.
// DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).

fn max_coins(grid: &[Vec<i32>]) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0i32; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let mut best = 0;
            if i > 0 {
                best = best.max(dp[i - 1][j]);
            }
            if j > 0 {
                best = best.max(dp[i][j - 1]);
            }
            dp[i][j] = grid[i][j] + best;
        }
    }
    dp[rows - 1][cols - 1]
}

fn main() {
    let matrix = vec![
        vec![0, 3, 1, 1],
        vec![2, 0, 0, 4],
        vec![1, 5, 3, 1],
    ];
    println!("{}", max_coins(&matrix));
}
