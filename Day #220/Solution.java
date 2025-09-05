// Day 220: Optimal coin-picking game (first player guaranteed max).
// Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
// Time O(n^2), Space O(n^2).
public class Solution {
    static int maxCoins(int[] v) {
        int n = v.length;
        if (n == 0) return 0;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = v[i];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                int innerLeft = (i + 2 <= j) ? dp[i + 2][j] : 0;
                int innerMid = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
                int innerRight = (i <= j - 2) ? dp[i][j - 2] : 0;
                int takeI = v[i] + Math.min(innerLeft, innerMid);
                int takeJ = v[j] + Math.min(innerMid, innerRight);
                dp[i][j] = Math.max(takeI, takeJ);
            }
        }
        return dp[0][n - 1];
    }

    public static void main(String[] args) {
        System.out.println(maxCoins(new int[]{8, 15, 3, 7})); // 22
    }
}
