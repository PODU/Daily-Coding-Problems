// Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
// j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).
import java.util.*;

public class Solution {
    static int numSquares(int n) {
        int[] dp = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int i = 1; i <= n; i++)
            for (int j = 1; j * j <= i; j++)
                dp[i] = Math.min(dp[i], dp[i - j * j] + 1);
        return dp[n];
    }

    public static void main(String[] args) {
        System.out.println(numSquares(13)); // 2
        System.out.println(numSquares(27)); // 3
    }
}
