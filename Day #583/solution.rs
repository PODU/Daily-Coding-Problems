// Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
fn max_coins(grid: &Vec<Vec<i32>>) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    let mut dp = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            let mut best = 0;
            if i > 0 {
                best = best.max(dp[i - 1][j]);
            }
            if j > 0 {
                best = best.max(dp[i][j - 1]);
            }
            dp[i][j] = grid[i][j] + if i == 0 && j == 0 { 0 } else { best };
        }
    }
    dp[r - 1][c - 1]
}

fn main() {
    let grid = vec![
        vec![0, 3, 1, 1],
        vec![2, 0, 0, 4],
        vec![1, 5, 3, 1],
    ];
    let result = max_coins(&grid);
    assert_eq!(result, 12);
    println!("The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = {} coins.", result);
}
