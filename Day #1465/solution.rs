// Optimal coin game via interval DP. dp[i][j] = best score first player can guarantee on coins[i..j].
// Time O(n^2), Space O(n^2).
fn coin_game(v: &[i64]) -> i64 {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i64; n]; n];
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
    println!("{}", coin_game(&[8, 15, 3, 7]));
    println!("{}", coin_game(&[2, 2, 2, 2]));
}
