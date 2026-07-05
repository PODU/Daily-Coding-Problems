// Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
// factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.
public class Solution {
    static int minSteps(int N) {
        int[] dp = new int[N + 1];
        for (int i = 2; i <= N; i++) {
            dp[i] = dp[i - 1] + 1;                  // decrement step
            for (int a = 2; (long) a * a <= i; a++) {
                if (i % a == 0) dp[i] = Math.min(dp[i], dp[i / a] + 1); // jump to larger factor
            }
        }
        return dp[N];
    }

    public static void main(String[] args) {
        System.out.println(minSteps(100)); // -> 5
    }
}
