// Day 158: Count paths (right/down only) avoiding walls. DP: dp[j] = ways into a
// free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).

fn count_paths(grid: &[Vec<i32>]) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut dp = vec![0u64; m];
    dp[0] = if grid[0][0] == 0 { 1 } else { 0 };
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                dp[j] = 0;
            } else if j > 0 {
                dp[j] += dp[j - 1];
            }
        }
    }
    dp[m - 1]
}

fn main() {
    let grid = vec![
        vec![0, 0, 1],
        vec![0, 0, 1],
        vec![1, 0, 0],
    ];
    println!("{}", count_paths(&grid)); // 2
}
