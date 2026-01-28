// Day 975: Count valid playlists of length N from M songs, each used >=1, repeats B apart.
// Approach: DP dp[i][j]=playlists length i with j distinct songs. Time O(N*M), Space O(N*M).
const MOD: i64 = 1_000_000_007;

fn num_playlists(n: usize, m: usize, b: i64) -> i64 {
    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j - 1] * (m - (j - 1)) as i64 % MOD;
            let rep = std::cmp::max(j as i64 - b, 0);
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * rep) % MOD;
        }
    }
    dp[n][m]
}

fn main() {
    println!("{}", num_playlists(3, 3, 1)); // 6
}
