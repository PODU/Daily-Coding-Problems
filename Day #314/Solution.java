// Day 314: Min broadcast range = max over listeners of distance to nearest tower.
// Sort towers, binary search each listener. O((N+M) log M).
import java.util.*;
public class Solution {
    static int minRange(int[] listeners, int[] towers) {
        Arrays.sort(towers);
        int ans = 0;
        for (int L : listeners) {
            int idx = Arrays.binarySearch(towers, L);
            if (idx < 0) idx = -idx - 1; // insertion point
            int best = Integer.MAX_VALUE;
            if (idx < towers.length) best = Math.min(best, towers[idx] - L);
            if (idx > 0) best = Math.min(best, L - towers[idx - 1]);
            ans = Math.max(ans, best);
        }
        return ans;
    }
    public static void main(String[] a) {
        System.out.println(minRange(new int[]{1, 5, 11, 20}, new int[]{4, 8, 15})); // 5
    }
}
