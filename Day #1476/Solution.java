// Day 1476: Max profit from a single buy-then-sell.
// Track the minimum price so far and the best profit in one pass.
// Time O(N), Space O(1).
public class Solution {
    static int maxProfit(int[] prices) {
        int minPrice = Integer.MAX_VALUE, best = 0;
        for (int p : prices) {
            if (p < minPrice) minPrice = p;
            else if (p - minPrice > best) best = p - minPrice;
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(maxProfit(new int[]{9, 11, 8, 5, 7, 10}));  // 5
    }
}
