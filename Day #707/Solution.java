// Day 707: Min broadcast range. Sort towers; for each listener binary-search the
// nearest tower, answer is max of those min distances. Time O((N+M)logM).
import java.util.*;

public class Solution {
    static int minRange(int[] listeners, int[] towers) {
        Arrays.sort(towers);
        int ans = 0;
        for (int x : listeners) {
            int idx = Arrays.binarySearch(towers, x);
            if (idx < 0) idx = -idx - 1; // insertion point
            int best = Integer.MAX_VALUE;
            if (idx < towers.length) best = Math.min(best, towers[idx] - x);
            if (idx > 0) best = Math.min(best, x - towers[idx - 1]);
            ans = Math.max(ans, best);
        }
        return ans;
    }

    public static void main(String[] args) {
        System.out.println(minRange(new int[]{1, 5, 11, 20}, new int[]{4, 8, 15}));
    }
}
