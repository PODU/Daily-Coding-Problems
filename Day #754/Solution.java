// Day 754: Optimal coin game (interval DP / minimax).
// dp[i][j] = max value first player guarantees from coins[i..j].
// Time: O(n^2), Space: O(n^2).
public class Solution {
    static long maxCoins(int[] v) {
        int n = v.length;
        if (n == 0) return 0;
        long[][] dp = new long[n][n];
        long[] pre = new long[n + 1];
        for (int i = 0; i < n; i++) pre[i + 1] = pre[i] + v[i];

        for (int i = 0; i < n; i++) dp[i][i] = v[i];
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                long takeLeft  = v[i] + (pre[j + 1] - pre[i + 1]) - dp[i + 1][j];
                long takeRight = v[j] + (pre[j] - pre[i]) - dp[i][j - 1];
                dp[i][j] = Math.max(takeLeft, takeRight);
            }
        return dp[0][n - 1];
    }

    public static void main(String[] args) {
        int[] coins = {8, 15, 3, 7};
        System.out.println(maxCoins(coins));  // 22
    }
}
