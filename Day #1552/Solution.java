// Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).
public class Solution {
    static int maxProfit(int[] prices) {
        if (prices.length == 0) return 0;
        int minPrice = prices[0], best = 0;
        for (int p : prices) {
            minPrice = Math.min(minPrice, p);
            best = Math.max(best, p - minPrice);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] prices = {9, 11, 8, 5, 7, 10};
        System.out.println(maxProfit(prices));
    }
}
