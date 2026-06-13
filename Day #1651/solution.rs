// Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
// with a buffer of B songs between repeats.
// DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
// Time O(N*M), Space O(N*M).
const MOD: i64 = 1_000_000_007;

fn num_playlists(n: usize, m: usize, b: i64) -> i64 {
    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j - 1] * (m as i64 - (j as i64 - 1)) % MOD;
            let rem = std::cmp::max(j as i64 - b, 0);
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * rem) % MOD;
        }
    }
    dp[n][m]
}

fn main() {
    // N=3, M=2, B=0 -> 6
    println!("{}", num_playlists(3, 2, 0));
}
