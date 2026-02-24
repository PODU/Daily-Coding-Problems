// Day 1121 - Max stock profit with a transaction fee, unlimited transactions
// State machine DP: best cash (not holding) and best hold. Time: O(n), Space: O(1).
public class Solution {
    static int maxProfit(int[] prices, int fee) {
        int cash = 0, hold = -prices[0];
        for (int i = 1; i < prices.length; i++) {
            cash = Math.max(cash, hold + prices[i] - fee);
            hold = Math.max(hold, cash - prices[i]);
        }
        return cash;
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(new int[]{1, 3, 2, 8, 4, 10}, 2)); // 9
    }
}
