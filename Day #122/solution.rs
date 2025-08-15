// Day 122: Max coins from top-left to bottom-right moving right/down.
// DP O(R*C) time and space, with path reconstruction (prefer left on ties).
fn main() {
    let g = vec![vec![0, 3, 1, 1], vec![2, 0, 0, 4], vec![1, 5, 3, 1]];
    let r = g.len();
    let c = g[0].len();
    let mut dp = vec![vec![0i64; c]; r];
    for i in 0..r {
        for j in 0..c {
            let best = if i == 0 && j == 0 {
                0
            } else if i == 0 {
                dp[i][j - 1]
            } else if j == 0 {
                dp[i - 1][j]
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
            dp[i][j] = g[i][j] + best;
        }
    }
    let mut path: Vec<i64> = Vec::new();
    let (mut i, mut j) = (r - 1, c - 1);
    while i > 0 || j > 0 {
        path.push(g[i][j]);
        if i == 0 {
            j -= 1;
        } else if j == 0 {
            i -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    path.push(g[0][0]);
    path.reverse();
    let parts: Vec<String> = path.iter().map(|v| v.to_string()).collect();
    println!(
        "The most we can collect is {} = {} coins.",
        parts.join(" + "),
        dp[r - 1][c - 1]
    );
}
