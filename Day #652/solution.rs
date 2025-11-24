// Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
// dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
const MOD: i64 = 1_000_000_007;

fn count_playlists(n: usize, m: usize, b: usize) -> i64 {
    let mut prev = vec![0i64; m + 1];
    prev[0] = 1;
    for _ in 1..=n {
        let mut cur = vec![0i64; m + 1];
        for j in 1..=m {
            cur[j] = prev[j - 1] * (m - (j - 1)) as i64 % MOD;
            let mult = if j > b { (j - b) as i64 } else { 0 };
            cur[j] = (cur[j] + prev[j] * mult) % MOD;
        }
        prev = cur;
    }
    prev[m]
}

fn main() {
    let (n, m, b) = (3, 3, 1);
    println!("{}", count_playlists(n, m, b));
}
