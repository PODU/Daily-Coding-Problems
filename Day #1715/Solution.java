// DP states cash/hold; fee charged once per buy-sell on sell. Time O(n), Space O(1).
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
        int[] prices = {1, 3, 2, 8, 4, 10};
        int fee = 2;
        System.out.println(maxProfit(prices, fee));
    }
}
