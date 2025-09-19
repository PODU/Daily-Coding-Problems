// Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
// Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<Integer, int[]> preferences = new LinkedHashMap<>();
        preferences.put(0, new int[]{0,1,3,6});
        preferences.put(1, new int[]{1,4,7});
        preferences.put(2, new int[]{2,4,7,5});
        preferences.put(3, new int[]{3,2,5});
        preferences.put(4, new int[]{5,8});

        int m = preferences.size();
        int full = (1 << m) - 1;

        Map<Integer,Integer> drinkMask = new HashMap<>();
        for (Map.Entry<Integer,int[]> e : preferences.entrySet()) {
            int cust = e.getKey();
            for (int d : e.getValue())
                drinkMask.merge(d, 1 << cust, (a,b) -> a | b);
        }

        int[] dp = new int[full + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        Queue<Integer> q = new ArrayDeque<>();
        q.add(0);
        while (!q.isEmpty()) {
            int mask = q.poll();
            if (mask == full) continue;
            for (int dm : drinkMask.values()) {
                int nm = mask | dm;
                if (dp[nm] > dp[mask] + 1) {
                    dp[nm] = dp[mask] + 1;
                    q.add(nm);
                }
            }
        }
        System.out.println(dp[full]);
    }
}
