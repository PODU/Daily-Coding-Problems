// Optimal coin game via interval DP. dp[i][j] = best score first player can guarantee on coins[i..j].
// Time O(n^2), Space O(n^2).
public class Solution {
    static int coinGame(int[] v) {
        int n = v.length;
        if (n == 0) return 0;
        int[][] dp = new int[n][n];
        for (int len = 1; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                int a = (i + 2 <= j) ? dp[i + 2][j] : 0;
                int b = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
                int c = (i <= j - 2) ? dp[i][j - 2] : 0;
                int takeFirst = v[i] + Math.min(a, b);
                int takeLast = v[j] + Math.min(b, c);
                dp[i][j] = Math.max(takeFirst, takeLast);
            }
        }
        return dp[0][n - 1];
    }

    public static void main(String[] args) {
        System.out.println(coinGame(new int[]{8, 15, 3, 7}));
        System.out.println(coinGame(new int[]{2, 2, 2, 2}));
    }
}
