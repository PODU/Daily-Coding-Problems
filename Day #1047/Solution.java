// Time-versioned map: per key a TreeMap<time,value>; get uses floorKey for floor.
// set/get O(log n). Setting same key+time overwrites.
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
            Integer fk = m.floorKey(time);
            return fk == null ? null : m.get(fk);
        }
    }

    static String show(Integer v) { return v == null ? "null" : v.toString(); }

    public static void main(String[] args) {
        // README presents three logical blocks; each starts from its own map state.
        TimeMap d = new TimeMap();
        d.set(1, 1, 0);
        d.set(1, 2, 2);
        System.out.println("d.get(1, 1) -> " + show(d.get(1, 1)));
        System.out.println("d.get(1, 3) -> " + show(d.get(1, 3)));

        d = new TimeMap();
        d.set(1, 1, 5);
        System.out.println("d.get(1, 0) -> " + show(d.get(1, 0)));
        System.out.println("d.get(1, 10) -> " + show(d.get(1, 10)));

        d = new TimeMap();
        d.set(1, 1, 0);
        d.set(1, 2, 0); // overwrite same key+time -> value 2
        System.out.println("d.get(1, 0) -> " + show(d.get(1, 0)));
    }
}
