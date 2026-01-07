// Day 866: Max profit with at most k buy/sell transactions.
// Approach: DP with buy[j]/sell[j] rolling arrays (or greedy when k >= n/2).
// Time: O(n*k), Space: O(k).
import java.util.*;

public class Solution {
    static int maxProfit(int k, int[] prices) {
        int n = prices.length;
        if (n == 0 || k == 0) return 0;
        if (k >= n / 2) {
            int profit = 0;
            for (int i = 1; i < n; i++) if (prices[i] > prices[i-1]) profit += prices[i] - prices[i-1];
            return profit;
        }
        int[] buy = new int[k + 1], sell = new int[k + 1];
        Arrays.fill(buy, Integer.MIN_VALUE);
        for (int p : prices)
            for (int j = 1; j <= k; j++) {
                buy[j] = Math.max(buy[j], sell[j-1] - p);
                sell[j] = Math.max(sell[j], buy[j] + p);
            }
        return sell[k];
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(2, new int[]{5, 2, 4, 0, 1})); // 3
    }
}
