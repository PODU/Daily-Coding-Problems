// Max profit, unlimited transactions with a fixed fee per completed sale.
// DP states cash/hold tracked greedily. Time O(n), Space O(1).
public class Solution {
    static int maxProfit(int[] prices, int fee) {
        long cash = 0, hold = Long.MIN_VALUE / 4;
        for (int p : prices) {
            hold = Math.max(hold, cash - p);
            cash = Math.max(cash, hold + p - fee);
        }
        return (int) cash;
    }

    public static void main(String[] args) {
        int[] prices = {1, 3, 2, 8, 4, 10};
        int fee = 2;
        System.out.println(maxProfit(prices, fee));
    }
}
