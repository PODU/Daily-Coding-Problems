// Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
// dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
public class Solution {
    static final long MOD = 1000000007L;

    static long countPlaylists(int N, int M, int B) {
        long[] prev = new long[M + 1];
        long[] cur = new long[M + 1];
        prev[0] = 1;
        for (int i = 1; i <= N; i++) {
            java.util.Arrays.fill(cur, 0);
            for (int j = 1; j <= M; j++) {
                cur[j] = (prev[j - 1] * (M - (j - 1))) % MOD;
                cur[j] = (cur[j] + prev[j] * Math.max(j - B, 0)) % MOD;
            }
            long[] tmp = prev; prev = cur; cur = tmp;
        }
        return prev[M];
    }

    public static void main(String[] args) {
        int N = 3, M = 3, B = 1;
        System.out.println(countPlaylists(N, M, B));
    }
}
