// Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.
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
        System.out.println(maxProfit(new int[]{9, 11, 8, 5, 7, 10}));
    }
}
