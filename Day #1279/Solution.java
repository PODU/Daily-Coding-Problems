// Day 1279: Lazy bartender = minimum set cover over customers.
// DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
import java.util.*;

public class Solution {
    static int minDrinks(int[][] prefs) {
        int C = prefs.length;
        Map<Integer, Integer> drinkMask = new HashMap<>();
        for (int cust = 0; cust < C; ++cust)
            for (int d : prefs[cust])
                drinkMask.merge(d, 1 << cust, (a, b) -> a | b);
        int full = (1 << C) - 1;
        int[] dp = new int[1 << C];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int mask = 0; mask <= full; ++mask) {
            if (dp[mask] == Integer.MAX_VALUE) continue;
            for (int dm : drinkMask.values()) {
                int nm = mask | dm;
                if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
            }
        }
        return dp[full];
    }

    public static void main(String[] args) {
        int[][] prefs = {{0, 1, 3, 6}, {1, 4, 7}, {2, 4, 7, 5}, {3, 2, 5}, {5, 8}};
        System.out.println(minDrinks(prefs)); // 2
    }
}
