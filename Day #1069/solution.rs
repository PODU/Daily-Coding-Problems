// Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.

fn coin_game(v: &[i32]) -> i32 {
    let n = v.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = v[i];
    }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            let a = if i + 2 <= j { dp[i + 2][j] } else { 0 };
            let b = if i + 1 < j { dp[i + 1][j - 1] } else { 0 };
            let take_i = v[i] + a.min(b);

            let c = if i + 1 < j { dp[i + 1][j - 1] } else { 0 };
            let d = if j >= 2 && i + 2 <= j { dp[i][j - 2] } else { 0 };
            let take_j = v[j] + c.min(d);

            dp[i][j] = take_i.max(take_j);
        }
    }
    dp[0][n - 1]
}

fn main() {
    let a = vec![8, 15, 3, 7];
    println!("Max guaranteed: {}", coin_game(&a));
    let b = vec![2, 2, 2, 2];
    println!("Max guaranteed: {}", coin_game(&b));
}
