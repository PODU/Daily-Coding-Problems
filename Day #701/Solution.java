// Day 701: Minimum drinks to satisfy every customer (minimum set cover).
// Approach: each drink -> bitmask of customers it satisfies; DP over customer
// masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).
import java.util.*;

public class Solution {
    static int minDrinks(Map<Integer, int[]> prefs) {
        int C = prefs.size();
        Map<Integer, Integer> idx = new HashMap<>();
        for (int cust : prefs.keySet()) idx.put(cust, idx.size());
        Map<Integer, Integer> drinkMask = new HashMap<>();
        for (Map.Entry<Integer, int[]> e : prefs.entrySet())
            for (int d : e.getValue())
                drinkMask.merge(d, 1 << idx.get(e.getKey()), (a, b) -> a | b);
        int full = (1 << C) - 1;
        int[] dp = new int[full + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int mask = 0; mask <= full; mask++) {
            if (dp[mask] == Integer.MAX_VALUE) continue;
            for (int dm : drinkMask.values()) {
                int nm = mask | dm;
                if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
            }
        }
        return dp[full];
    }

    public static void main(String[] args) {
        Map<Integer, int[]> prefs = new LinkedHashMap<>();
        prefs.put(0, new int[]{0, 1, 3, 6});
        prefs.put(1, new int[]{1, 4, 7});
        prefs.put(2, new int[]{2, 4, 7, 5});
        prefs.put(3, new int[]{3, 2, 5});
        prefs.put(4, new int[]{5, 8});
        System.out.println(minDrinks(prefs)); // 2
    }
}
