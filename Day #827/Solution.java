// Day 827: Min broadcast range.
// Sort towers; for each listener binary-search nearest tower, take min distance;
// answer = max over listeners. Time O((N+M) log M), Space O(1).
import java.util.Arrays;

public class Solution {
    static long minBroadcastRange(long[] listeners, long[] towers) {
        Arrays.sort(towers);
        long ans = 0;
        for (long l : listeners) {
            long best = Long.MAX_VALUE;
            int i = lowerBound(towers, l);
            if (i < towers.length) best = Math.min(best, towers[i] - l);
            if (i > 0) best = Math.min(best, l - towers[i - 1]);
            ans = Math.max(ans, best);
        }
        return ans;
    }

    static int lowerBound(long[] a, long key) {
        int lo = 0, hi = a.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (a[mid] < key) lo = mid + 1; else hi = mid;
        }
        return lo;
    }

    public static void main(String[] args) {
        System.out.println(minBroadcastRange(new long[]{1, 5, 11, 20}, new long[]{4, 8, 15}));
    }
}
