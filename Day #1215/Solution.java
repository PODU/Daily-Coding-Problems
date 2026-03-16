// Day 1215: Min steps to reduce N to 1 (decrement, or replace by larger factor).
// DP: dp[i] = 1 + min(dp[i-1], dp[i/d] for divisors d). Time O(N sqrt N), Space O(N).
public class Solution {
    static int minSteps(int n) {
        int[] dp = new int[n + 1];
        dp[1] = 0;
        for (int i = 2; i <= n; i++) {
            dp[i] = dp[i - 1] + 1;
            for (int d = 2; (long) d * d <= i; d++)
                if (i % d == 0) dp[i] = Math.min(dp[i], dp[i / d] + 1);
        }
        return dp[n];
    }

    public static void main(String[] args) {
        System.out.println(minSteps(100)); // 5
    }
}
