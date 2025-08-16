// Day 130: Max profit with at most k buy/sell transactions.
// DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).
import java.util.*;

public class Solution {
    static int maxProfit(int k, int[] p) {
        int n = p.length;
        if (n == 0 || k == 0) return 0;
        if (k >= n / 2) {
            int prof = 0;
            for (int i = 1; i < n; i++)
                if (p[i] > p[i - 1]) prof += p[i] - p[i - 1];
            return prof;
        }
        int[] buy = new int[k + 1], sell = new int[k + 1];
        Arrays.fill(buy, Integer.MIN_VALUE);
        for (int price : p)
            for (int j = 1; j <= k; j++) {
                buy[j] = Math.max(buy[j], sell[j - 1] - price);
                sell[j] = Math.max(sell[j], buy[j] + price);
            }
        return sell[k];
    }

    public static void main(String[] args) {
        int[] prices = {5, 2, 4, 0, 1};
        System.out.println(maxProfit(2, prices)); // 3
    }
}
