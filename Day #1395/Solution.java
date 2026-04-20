// LRU cache via LinkedHashMap (access-order) with removeEldestEntry; get/set O(1). O(1) time, O(n) space.
import java.util.LinkedHashMap;
import java.util.Map;

public class Solution {
    static class LRUCache {
        private final int cap;
        private final LinkedHashMap<Integer, Integer> map;

        LRUCache(int capacity) {
            this.cap = capacity;
            this.map = new LinkedHashMap<Integer, Integer>(16, 0.75f, true) {
                @Override
                protected boolean removeEldestEntry(Map.Entry<Integer, Integer> eldest) {
                    return size() > cap;
                }
            };
        }

        // returns null if missing
        Integer get(int key) {
            return map.get(key);
        }

        void set(int key, int value) {
            map.put(key, value);
        }
    }

    public static void main(String[] args) {
        LRUCache cache = new LRUCache(2);
        cache.set(1, 1);
        cache.set(2, 2);
        System.out.println(fmt(cache.get(1)));   // 1
        cache.set(3, 3);                          // evicts key 2
        System.out.println(fmt(cache.get(2)));   // null
        cache.set(4, 4);                          // evicts key 1
        System.out.println(fmt(cache.get(1)));   // null
        System.out.println(fmt(cache.get(3)));   // 3
        System.out.println(fmt(cache.get(4)));   // 4
    }

    static String fmt(Integer v) {
        return v == null ? "null" : v.toString();
    }
}
