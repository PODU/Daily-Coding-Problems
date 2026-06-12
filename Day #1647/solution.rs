// Coins-in-a-row: dp[i][j] = max value first-to-move guarantees from coins[i..j],
// dp[i][j]=max(v[i]+min(dp[i+2][j],dp[i+1][j-1]), v[j]+min(dp[i+1][j-1],dp[i][j-2])). Time/Space O(n^2).

fn max_guaranteed(v: &[i32]) -> i32 {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i32; n]; n];
    for len in 1..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            let a = if i + 2 <= j { dp[i + 2][j] } else { 0 };
            let b = if i + 1 <= j.wrapping_sub(1) && j >= 1 { dp[i + 1][j - 1] } else { 0 };
            let c = if j >= 2 && i <= j - 2 { dp[i][j - 2] } else { 0 };
            let take_first = v[i] + a.min(b);
            let take_last = v[j] + b.min(c);
            dp[i][j] = take_first.max(take_last);
        }
    }
    dp[0][n - 1]
}

fn main() {
    let coins = [3, 9, 1, 2];
    println!("{}", max_guaranteed(&coins));
}
