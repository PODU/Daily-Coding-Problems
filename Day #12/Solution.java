// Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
// Time: O(N*|X|), Space: O(N).
public class Solution {
    static long staircase(int n, int[] X) {
        long[] dp = new long[n + 1];
        dp[0] = 1;
        for (int i = 1; i <= n; i++)
            for (int x : X)
                if (i - x >= 0) dp[i] += dp[i - x];
        return dp[n];
    }

    public static void main(String[] args) {
        System.out.println(staircase(4, new int[]{1, 2})); // 5
    }
}
