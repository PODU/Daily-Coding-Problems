// Day 632: Deduce coin denominations from a "ways to make change" array.
// Approach: reverse coin-change DP. If ways[i] exceeds count reachable with
// coins found so far, i is itself a denomination.
// Time: O(N * D), Space: O(N).
import java.util.*;

public class Solution {
    static List<Integer> findDenominations(long[] ways) {
        int n = ways.length;
        long[] dp = new long[n];
        dp[0] = 1;
        List<Integer> coins = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (dp[i] < ways[i]) {
                coins.add(i);
                for (int j = i; j < n; j++) dp[j] += dp[j - i];
            }
        }
        return coins;
    }

    public static void main(String[] args) {
        long[] ways = {1, 0, 1, 1, 2};
        List<Integer> coins = findDenominations(ways);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < coins.size(); i++) {
            if (i + 1 == coins.size() && coins.size() > 1) sb.append("and ");
            sb.append(coins.get(i));
            if (i + 1 < coins.size()) sb.append(", ");
        }
        System.out.println(sb);
    }
}
