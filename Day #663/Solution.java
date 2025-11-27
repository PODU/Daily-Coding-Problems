// Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
// record O(log n), total O(1), range O(log n).
// Limited-memory follow-up: bucket hits into fixed time windows and store (window,count).
import java.util.*;

public class Solution {
    static class HitCounter {
        // value -> count, kept sorted by key
        TreeMap<Long, Integer> ts = new TreeMap<>();
        long size = 0;

        void record(long t) { ts.merge(t, 1, Integer::sum); size++; }
        long total() { return size; }
        long range(long lo, long hi) {
            long c = 0;
            for (int v : ts.subMap(lo, true, hi, true).values()) c += v;
            return c;
        }
    }

    public static void main(String[] args) {
        HitCounter h = new HitCounter();
        for (long t : new long[]{1, 2, 2, 5, 9, 10}) h.record(t);
        System.out.println("total: " + h.total());           // 6
        System.out.println("range(2,9): " + h.range(2, 9));   // 4
    }
}
