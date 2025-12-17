// Day 754: Optimal coin game (interval DP / minimax).
// dp[i][j] = max value first player guarantees from coins[i..j].
// Time: O(n^2), Space: O(n^2).
fn max_coins(v: &[i64]) -> i64 {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    let mut pre = vec![0i64; n + 1];
    for i in 0..n {
        pre[i + 1] = pre[i] + v[i];
    }
    let mut dp = vec![vec![0i64; n]; n];
    for i in 0..n {
        dp[i][i] = v[i];
    }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            let take_left = v[i] + (pre[j + 1] - pre[i + 1]) - dp[i + 1][j];
            let take_right = v[j] + (pre[j] - pre[i]) - dp[i][j - 1];
            dp[i][j] = take_left.max(take_right);
        }
    }
    dp[0][n - 1]
}

fn main() {
    let coins = [8i64, 15, 3, 7];
    println!("{}", max_coins(&coins)); // 22
}
