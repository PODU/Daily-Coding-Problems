// Day 132: HitCounter (record, total, range).
// TreeMap timestamp->count: total O(1) counter, range O(log n) via subMap sum.
// Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
import java.util.*;

public class Solution {
    static class HitCounter {
        TreeMap<Long, Integer> counts = new TreeMap<>();
        int totalHits = 0;

        void record(long t) {
            counts.merge(t, 1, Integer::sum);
            totalHits++;
        }

        int total() { return totalHits; }

        int range(long lo, long hi) {
            int sum = 0;
            for (int c : counts.subMap(lo, true, hi, true).values()) sum += c;
            return sum;
        }
    }

    public static void main(String[] args) {
        HitCounter hc = new HitCounter();
        for (long t : new long[]{1, 1, 2, 3, 5, 8, 8, 10}) hc.record(t);
        System.out.println("total = " + hc.total());          // 8
        System.out.println("range(2, 8) = " + hc.range(2, 8)); // 5
    }
}
