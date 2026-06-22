// Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
// Time O(N^2), Space O(N).
import java.util.*;

public class Solution {
    static List<Integer> findDenominations(int[] ways) {
        int n = ways.length;
        long[] dp = new long[n];
        dp[0] = 1;
        List<Integer> coins = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (ways[i] != dp[i]) {
                coins.add(i);
                for (int j = i; j < n; j++) dp[j] += dp[j - i];
            }
        }
        return coins;
    }

    public static void main(String[] args) {
        int[] ways = {1, 0, 1, 1, 2};
        System.out.println(findDenominations(ways));
    }
}
