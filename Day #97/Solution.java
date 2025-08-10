// Day 97: Time-versioned map. Each key keeps a TreeMap time->value; get uses
// floorEntry to find the latest time <= query. set/get O(log n).
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

    public static void main(String[] args) {
        // The README's three blocks are independent scenarios (fresh maps).
        TimeMap a = new TimeMap();
        a.set(1, 1, 0); a.set(1, 2, 2);
        System.out.println(a.get(1, 1));   // 1
        System.out.println(a.get(1, 3));   // 2

        TimeMap b = new TimeMap();
        b.set(1, 1, 5);
        System.out.println(b.get(1, 0));   // null
        System.out.println(b.get(1, 10));  // 1

        TimeMap c = new TimeMap();
        c.set(1, 1, 0); c.set(1, 2, 0);
        System.out.println(c.get(1, 0));   // 2
    }
}
