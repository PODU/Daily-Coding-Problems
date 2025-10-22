// Min coins via DP over amounts (optimal for arbitrary denominations).
// Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).
import java.util.Arrays;

public class Solution {
    static int minCoins(int n, int[] coins) {
        final int INF = Integer.MAX_VALUE;
        int[] dp = new int[n + 1];
        Arrays.fill(dp, INF);
        dp[0] = 0;
        for (int a = 1; a <= n; a++) {
            for (int c : coins) {
                if (c <= a && dp[a - c] != INF) {
                    dp[a] = Math.min(dp[a], dp[a - c] + 1);
                }
            }
        }
        return dp[n];
    }

    public static void main(String[] args) {
        int[] coins = {1, 5, 10, 25};
        int n = 16;
        System.out.println(minCoins(n, coins));
    }
}
