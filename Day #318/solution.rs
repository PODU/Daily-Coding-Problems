// Count playlists of length N from M songs, each used >=1, gap >=B between repeats.
// DP over length x distinct songs (LeetCode 920). Time O(N*M), Space O(N*M).

const MOD: i64 = 1_000_000_007;

fn num_playlists(n: usize, m: usize, b: i64) -> i64 {
    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j - 1] * (m as i64 - (j as i64 - 1)) % MOD;
            let g = (j as i64 - b).max(0);
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * g) % MOD;
        }
    }
    dp[n][m]
}

fn main() {
    let (n, m, b) = (3usize, 3usize, 1i64);
    println!(
        "Number of valid playlists (N={}, M={}, B={}) = {}",
        n, m, b, num_playlists(n, m, b)
    );
}
