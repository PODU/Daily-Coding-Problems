// Day 193: Max profit, unlimited transactions, fee charged per completed transaction.
// State DP: cash (no stock) / hold (holding). Time O(n), Space O(1).
public class Solution {
    static long maxProfit(int[] prices, int fee) {
        if (prices.length == 0) return 0;
        long cash = 0, hold = -prices[0];
        for (int i = 1; i < prices.length; i++) {
            cash = Math.max(cash, hold + prices[i] - fee);
            hold = Math.max(hold, cash - prices[i]);
        }
        return cash;
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(new int[]{1, 3, 2, 8, 4, 10}, 2));
    }
}
