// HitCounter: maintain timestamps in a sorted list via binary-search insert. record O(n) shift,
// total = size O(1), range = upperBound - lowerBound via binary search O(log n).
// Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
// so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.
import java.util.*;

public class Solution {
    static class HitCounter {
        private final List<Long> ts = new ArrayList<>();

        void record(long t) {
            int pos = lowerBound(t);
            ts.add(pos, t);
        }

        int total() { return ts.size(); }

        int range(long lower, long upper) {
            return upperBound(upper) - lowerBound(lower);
        }

        // first index with ts[i] >= key
        private int lowerBound(long key) {
            int lo = 0, hi = ts.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (ts.get(mid) < key) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }

        // first index with ts[i] > key
        private int upperBound(long key) {
            int lo = 0, hi = ts.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (ts.get(mid) <= key) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }
    }

    public static void main(String[] args) {
        HitCounter hc = new HitCounter();
        hc.record(1);
        hc.record(2);
        hc.record(3);
        hc.record(2);
        System.out.println(hc.total());      // 4
        System.out.println(hc.range(2, 3));  // 3
    }
}
