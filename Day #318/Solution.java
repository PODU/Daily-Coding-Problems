// Count playlists of length N from M songs, each used >=1, gap >=B between repeats.
// DP over length x distinct songs (LeetCode 920). Time O(N*M), Space O(N*M).
public class Solution {
    static final long MOD = 1000000007L;

    static long numPlaylists(int N, int M, int B) {
        long[][] dp = new long[N + 1][M + 1];
        dp[0][0] = 1;
        for (int i = 1; i <= N; i++) {
            for (int j = 1; j <= M; j++) {
                dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD;
                dp[i][j] = (dp[i][j] + dp[i - 1][j] * Math.max(j - B, 0)) % MOD;
            }
        }
        return dp[N][M];
    }

    public static void main(String[] args) {
        int N = 3, M = 3, B = 1;
        System.out.println("Number of valid playlists (N=" + N + ", M=" + M
                + ", B=" + B + ") = " + numPlaylists(N, M, B));
    }
}
