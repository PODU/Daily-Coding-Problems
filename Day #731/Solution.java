// Day 731: Max profit from a single buy-then-sell.
// Approach: Track running minimum price and best profit in one pass.
// Time: O(n), Space: O(1).
public class Solution {
    static int maxProfit(int[] prices) {
        int minPrice = Integer.MAX_VALUE, best = 0;
        for (int p : prices) {
            minPrice = Math.min(minPrice, p);
            best = Math.max(best, p - minPrice);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] prices = {9, 11, 8, 5, 7, 10};
        System.out.println(maxProfit(prices)); // 5
    }
}
