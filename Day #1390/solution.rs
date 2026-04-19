// Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.

fn main() {
    let m = vec![
        vec![0, 3, 1, 1],
        vec![2, 0, 0, 4],
        vec![1, 5, 3, 1],
    ];
    let r = m.len();
    let c = m[0].len();
    let mut dp = vec![vec![0i32; c]; r];
    for i in 0..r {
        for j in 0..c {
            let mut best = 0;
            if i > 0 { best = best.max(dp[i - 1][j]); }
            if j > 0 { best = best.max(dp[i][j - 1]); }
            dp[i][j] = m[i][j] + if i == 0 && j == 0 { 0 } else { best };
        }
    }
    println!("{}", dp[r - 1][c - 1]); // 12
}
