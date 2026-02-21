// Day 1111 - Lazy bartender (minimum set cover)
// Approach: exact set cover via DP over bitmask of covered customers.
// Time: O(D * 2^C), Space: O(2^C).
import java.util.*;

public class Solution {
    static int minDrinks(Map<Integer, int[]> preferences) {
        List<Integer> customers = new ArrayList<>(preferences.keySet());
        Collections.sort(customers);
        int n = customers.size();
        Map<Integer, Integer> cidx = new HashMap<>();
        for (int i = 0; i < n; i++) cidx.put(customers.get(i), i);

        Map<Integer, Integer> drinkMask = new HashMap<>();
        for (Map.Entry<Integer, int[]> e : preferences.entrySet())
            for (int d : e.getValue())
                drinkMask.merge(d, 1 << cidx.get(e.getKey()), (a, b) -> a | b);

        int full = (1 << n) - 1;
        int INF = Integer.MAX_VALUE;
        int[] dp = new int[1 << n];
        Arrays.fill(dp, INF);
        dp[0] = 0;
        for (int s = 0; s < (1 << n); s++) {
            if (dp[s] == INF) continue;
            for (int m : drinkMask.values()) {
                int ns = s | m;
                if (dp[ns] > dp[s] + 1) dp[ns] = dp[s] + 1;
            }
        }
        return dp[full];
    }

    public static void main(String[] args) {
        Map<Integer, int[]> preferences = new HashMap<>();
        preferences.put(0, new int[]{0, 1, 3, 6});
        preferences.put(1, new int[]{1, 4, 7});
        preferences.put(2, new int[]{2, 4, 7, 5});
        preferences.put(3, new int[]{3, 2, 5});
        preferences.put(4, new int[]{5, 8});
        System.out.println(minDrinks(preferences)); // 2
    }
}
