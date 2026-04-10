// Day 1333: Count right/down paths from top-left to bottom-right avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. O(N*M) time, O(M) space.
public class Solution {
    static long countPaths(int[][] g) {
        int n = g.length, m = g[0].length;
        long[] dp = new long[m];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++) {
                if (g[i][j] == 1) { dp[j] = 0; continue; }
                if (i == 0 && j == 0) dp[j] = 1;
                else dp[j] = (i > 0 ? dp[j] : 0) + (j > 0 ? dp[j - 1] : 0);
            }
        return dp[m - 1];
    }

    public static void main(String[] args) {
        int[][] g = {{0,0,1},{0,0,1},{1,0,0}};
        System.out.println(countPaths(g)); // 2
    }
}
