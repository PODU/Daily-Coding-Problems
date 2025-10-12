// Day 415: Max stock profit, unlimited transactions with a per-transaction fee.
// DP two states: cash (no stock) and hold (holding). Time O(N), Space O(1).
public class Solution {
    static int maxProfit(int[] prices, int fee) {
        if (prices.length == 0) return 0;
        long cash = 0, hold = -prices[0];
        for (int i = 1; i < prices.length; i++) {
            cash = Math.max(cash, hold + prices[i] - fee);
            hold = Math.max(hold, cash - prices[i]);
        }
        return (int) cash;
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(new int[]{1, 3, 2, 8, 4, 10}, 2)); // 9
    }
}
