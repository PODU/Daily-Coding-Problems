// Day 1293: Count valid playlists of length N from M songs, each used >=1, repeats >=B apart.
// DP: dp[i][j]=playlists of len i with j distinct songs. O(N*M) time, O(N*M) space (mod 1e9+7).
public class Solution {
    static final long MOD = 1000000007L;

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
        System.out.println(numPlaylists(3, 3, 1)); // 6
    }
}
