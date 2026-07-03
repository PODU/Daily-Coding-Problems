// Day 1756: HitCounter design.
// Store timestamps in a sorted list; total() O(1), range() via binary search O(log n).
// Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
// counts in a map/ring buffer) so memory is O(#buckets) instead of O(#hits).
import java.util.*;

public class Solution {
    static class HitCounter {
        private final ArrayList<Long> hits = new ArrayList<>(); // kept sorted

        void record(long timestamp) {
            int idx = upperBound(timestamp);
            hits.add(idx, timestamp);
        }
        int total() { return hits.size(); }
        int range(long lower, long upper) {
            return upperBoundVal(upper) - lowerBoundVal(lower);
        }
        // first index with value > timestamp
        private int upperBound(long key) {
            int lo = 0, hi = hits.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (hits.get(mid) <= key) lo = mid + 1; else hi = mid;
            }
            return lo;
        }
        private int upperBoundVal(long key) { return upperBound(key); }
        // first index with value >= key
        private int lowerBoundVal(long key) {
            int lo = 0, hi = hits.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (hits.get(mid) < key) lo = mid + 1; else hi = mid;
            }
            return lo;
        }
    }

    public static void main(String[] args) {
        HitCounter hc = new HitCounter();
        for (long t : new long[]{1, 2, 2, 5, 7, 9, 10}) hc.record(t);

        System.out.println("total() = " + hc.total());           // 7
        System.out.println("range(2, 7) = " + hc.range(2, 7));   // 4
        System.out.println("range(0, 10) = " + hc.range(0, 10)); // 7
    }
}
