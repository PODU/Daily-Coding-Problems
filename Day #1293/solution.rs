// Day 1293: Count valid playlists of length N from M songs, each used >=1, repeats >=B apart.
// DP: dp[i][j]=playlists of len i with j distinct songs. O(N*M) time, O(N*M) space (mod 1e9+7).
const MOD: u64 = 1000000007;

fn num_playlists(n: usize, m: usize, b: usize) -> u64 {
    let mut dp = vec![vec![0u64; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j - 1] * (m - (j - 1)) as u64 % MOD;
            let reuse = if j > b { (j - b) as u64 } else { 0 };
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * reuse) % MOD;
        }
    }
    dp[n][m]
}

fn main() {
    println!("{}", num_playlists(3, 3, 1)); // 6
}
