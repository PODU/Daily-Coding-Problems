// Day 408: Max profit with at most k stock transactions.
// Approach: DP tracking best buy/sell state per transaction in one pass.
// Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.
import java.util.*;

public class Solution {
    static int maxProfit(int k, int[] prices) {
        int n = prices.length;
        if (n == 0 || k == 0) return 0;
        // If k >= n/2, unlimited transactions are possible.
        if (k >= n / 2) {
            int profit = 0;
            for (int i = 1; i < n; i++)
                if (prices[i] > prices[i - 1]) profit += prices[i] - prices[i - 1];
            return profit;
        }
        int[] buy = new int[k + 1];
        int[] sell = new int[k + 1];
        Arrays.fill(buy, Integer.MIN_VALUE);
        for (int price : prices) {
            for (int t = 1; t <= k; t++) {
                buy[t] = Math.max(buy[t], sell[t - 1] - price);
                sell[t] = Math.max(sell[t], buy[t] + price);
            }
        }
        return sell[k];
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(2, new int[]{5, 2, 4, 0, 1})); // 3
    }
}
