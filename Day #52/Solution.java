// Day 52: LRU cache with hashmap + doubly linked list. O(1) get/set.
// Time: O(1) per op, Space: O(n).
import java.util.*;

public class Solution {
    static class LRUCache {
        private final int cap;
        private final LinkedHashMap<Integer, Integer> map;

        LRUCache(int n) {
            cap = n;
            map = new LinkedHashMap<>(16, 0.75f, true) { // access-order
                protected boolean removeEldestEntry(Map.Entry<Integer, Integer> e) {
                    return size() > cap;
                }
            };
        }

        Integer get(int key) { return map.getOrDefault(key, null); }

        void set(int key, int value) { map.put(key, value); }
    }

    public static void main(String[] args) {
        LRUCache c = new LRUCache(2);
        c.set(1, 1);
        c.set(2, 2);
        System.out.println(c.get(1)); // 1
        c.set(3, 3);                   // evicts key 2
        System.out.println(c.get(2)); // null
        c.set(4, 4);                   // evicts key 1
        System.out.println(c.get(1)); // null
        System.out.println(c.get(3)); // 3
        System.out.println(c.get(4)); // 4
    }
}
