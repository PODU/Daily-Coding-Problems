// HitCounter: keep timestamps in a sorted list (binary-insert); record O(n), total O(1), range via binary search (upper-lower).
import java.util.*;

public class Solution {
    static class HitCounter {
        private final List<Integer> hits = new ArrayList<>();
        private long cnt = 0;

        void record(int t) {
            int i = lowerBound(t);
            hits.add(i, t);
            cnt++;
        }

        long total() { return cnt; }

        long range(int lo, int hi) {
            return upperBound(hi) - lowerBound(lo);
        }

        private int lowerBound(int key) {
            int lo = 0, hi = hits.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (hits.get(mid) < key) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }

        private int upperBound(int key) {
            int lo = 0, hi = hits.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (hits.get(mid) <= key) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }
    }

    public static void main(String[] args) {
        HitCounter hc = new HitCounter();
        for (int t : new int[]{1, 1, 2, 3, 5, 8}) hc.record(t);
        System.out.println("total: " + hc.total());
        System.out.println("range(2,5): " + hc.range(2, 5));
    }
}
