// Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
// binary search for the most recent time <= query. set O(log n), get O(log n).
import java.util.*;

public class Solution {
    static class TimeMap {
        // key -> sorted list of [time, value]. Same time => later set overwrites.
        private final Map<Integer, List<int[]>> store = new HashMap<>();

        void set(int key, int value, int time) {
            List<int[]> v = store.computeIfAbsent(key, k -> new ArrayList<>());
            int lo = 0, hi = v.size(); // first index with time >= target
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (v.get(mid)[0] < time) lo = mid + 1; else hi = mid;
            }
            if (lo < v.size() && v.get(lo)[0] == time) v.get(lo)[1] = value; // overwrite
            else v.add(lo, new int[]{time, value});
        }

        // returns Integer value, or null if nothing set at/before time.
        Integer get(int key, int time) {
            List<int[]> v = store.get(key);
            if (v == null) return null;
            int lo = 0, hi = v.size() - 1, idx = -1;
            while (lo <= hi) {
                int mid = (lo + hi) >>> 1;
                if (v.get(mid)[0] <= time) { idx = mid; lo = mid + 1; }
                else hi = mid - 1;
            }
            return idx < 0 ? null : v.get(idx)[1];
        }
    }

    static void show(TimeMap d, int key, int time) {
        Integer val = d.get(key, time);
        System.out.println("get(" + key + "," + time + ") = " + (val == null ? "null" : val));
    }

    public static void main(String[] args) {
        TimeMap d1 = new TimeMap();
        d1.set(1, 1, 0); d1.set(1, 2, 2);
        show(d1, 1, 1);   // 1
        show(d1, 1, 3);   // 2

        TimeMap d2 = new TimeMap();
        d2.set(1, 1, 5);
        show(d2, 1, 0);   // null
        show(d2, 1, 10);  // 1

        TimeMap d3 = new TimeMap();
        d3.set(1, 1, 0); d3.set(1, 2, 0);
        show(d3, 1, 0);   // 2
    }
}
