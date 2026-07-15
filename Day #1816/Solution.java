// Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
// set O(log m), get O(log m) via floorKey. Space: O(total entries).
import java.util.*;

public class Solution {
    static class TimeMap {
        Map<Integer, TreeMap<Integer, Integer>> data = new HashMap<>();
        void set(int key, int value, int time) {
            data.computeIfAbsent(key, k -> new TreeMap<>()).put(time, value);
        }
        Integer get(int key, int time) {
            TreeMap<Integer, Integer> m = data.get(key);
            if (m == null) return null;
            Map.Entry<Integer, Integer> e = m.floorEntry(time);
            return e == null ? null : e.getValue();
        }
    }

    static void show(TimeMap d, int key, int time) {
        Integer r = d.get(key, time);
        System.out.println("get(" + key + ", " + time + ") = " + (r == null ? "null" : r));
    }

    public static void main(String[] args) {
        TimeMap d1 = new TimeMap(); d1.set(1,1,0); d1.set(1,2,2); show(d1,1,1); show(d1,1,3); // 1, 2
        TimeMap d2 = new TimeMap(); d2.set(1,1,5); show(d2,1,0); show(d2,1,10);                // null, 1
        TimeMap d3 = new TimeMap(); d3.set(1,1,0); d3.set(1,2,0); show(d3,1,0);                // 2
    }
}
