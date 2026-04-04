// Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
// DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.
public class Solution {
    static long climbWays(int n, int[] steps) {
        long[] dp = new long[n + 1];
        dp[0] = 1;
        for (int i = 1; i <= n; i++)
            for (int x : steps)
                if (i - x >= 0) dp[i] += dp[i - x];
        return dp[n];
    }

    public static void main(String[] args) {
        System.out.println(climbWays(4, new int[]{1, 2}));     // 5
        System.out.println(climbWays(10, new int[]{1, 3, 5})); // generalized
    }
}
