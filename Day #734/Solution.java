// Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
// Approach: per key a TreeMap time->value; get uses floorEntry.
// Time: set O(log n), get O(log n). Space: O(n).
import java.util.*;

public class Solution {
    static class TimeMap {
        Map<Integer, TreeMap<Integer, Integer>> store = new HashMap<>();
        void set(int key, int value, int time) {
            store.computeIfAbsent(key, k -> new TreeMap<>()).put(time, value);
        }
        Integer get(int key, int time) {
            TreeMap<Integer, Integer> m = store.get(key);
            if (m == null) return null;
            Map.Entry<Integer, Integer> e = m.floorEntry(time);
            return e == null ? null : e.getValue();
        }
    }

    static void show(Integer v) { System.out.println(v == null ? "null" : v.toString()); }

    public static void main(String[] args) {
        TimeMap d1 = new TimeMap(); d1.set(1, 1, 0); d1.set(1, 2, 2);
        show(d1.get(1, 1)); // 1
        show(d1.get(1, 3)); // 2
        TimeMap d2 = new TimeMap(); d2.set(1, 1, 5);
        show(d2.get(1, 0)); // null
        show(d2.get(1, 10)); // 1
        TimeMap d3 = new TimeMap(); d3.set(1, 1, 0); d3.set(1, 2, 0);
        show(d3.get(1, 0)); // 2
    }
}
