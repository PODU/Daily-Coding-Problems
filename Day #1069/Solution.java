// Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.
public class Solution {
    static int coinGame(int[] v) {
        int n = v.length;
        if (n == 0) return 0;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = v[i];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i <= n - len; i++) {
                int j = i + len - 1;
                int takeI = v[i] + Math.min(
                    (i+2 <= j ? dp[i+2][j] : 0),
                    (i+1 <= j-1 ? dp[i+1][j-1] : 0)
                );
                int takeJ = v[j] + Math.min(
                    (i+1 <= j-1 ? dp[i+1][j-1] : 0),
                    (i <= j-2 ? dp[i][j-2] : 0)
                );
                dp[i][j] = Math.max(takeI, takeJ);
            }
        }
        return dp[0][n-1];
    }

    public static void main(String[] args) {
        int[] a = {8, 15, 3, 7};
        System.out.println("Max guaranteed: " + coinGame(a));
        int[] b = {2, 2, 2, 2};
        System.out.println("Max guaranteed: " + coinGame(b));
    }
}
