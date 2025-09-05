// Day 220: Optimal coin-picking game (first player guaranteed max).
// Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
// Time O(n^2), Space O(n^2).
fn max_coins(v: &[i32]) -> i32 {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = v[i];
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let inner_left = if i + 2 <= j { dp[i + 2][j] } else { 0 };
            let inner_mid = if i + 1 <= j.wrapping_sub(1) && j >= 1 { dp[i + 1][j - 1] } else { 0 };
            let inner_right = if i + 2 <= j { dp[i][j - 2] } else { 0 };
            let take_i = v[i] + inner_left.min(inner_mid);
            let take_j = v[j] + inner_mid.min(inner_right);
            dp[i][j] = take_i.max(take_j);
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", max_coins(&[8, 15, 3, 7])); // 22
}
