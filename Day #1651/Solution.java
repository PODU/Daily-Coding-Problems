// Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
// with a buffer of B songs between repeats.
// DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
// Time O(N*M), Space O(N*M).
public class Solution {
    static final long MOD = 1_000_000_007L;

    static long numPlaylists(int N, int M, int B) {
        long[][] dp = new long[N + 1][M + 1];
        dp[0][0] = 1;
        for (int i = 1; i <= N; i++)
            for (int j = 1; j <= M; j++) {
                dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD;
                dp[i][j] = (dp[i][j] + dp[i - 1][j] * Math.max(j - B, 0)) % MOD;
            }
        return dp[N][M];
    }

    public static void main(String[] args) {
        // N=3, M=2, B=0 -> 6
        System.out.println(numPlaylists(3, 2, 0));
    }
}
