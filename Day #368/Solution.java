// Day 368: KV store with update/get/maxKey(val).
// kv maps key->value; byVal maps value->TreeSet of keys, so maxKey is last().
// update O(log n), get O(1) avg, maxKey O(log n).
import java.util.*;

public class Solution {
    static class KVStore {
        Map<Integer, Integer> kv = new HashMap<>();
        Map<Integer, TreeSet<Integer>> byVal = new HashMap<>();

        void update(int key, int val) {
            Integer old = kv.get(key);
            if (old != null) {
                TreeSet<Integer> set = byVal.get(old);
                set.remove(key);
                if (set.isEmpty()) byVal.remove(old);
            }
            kv.put(key, val);
            byVal.computeIfAbsent(val, k -> new TreeSet<>()).add(key);
        }

        Integer get(int key) { return kv.get(key); }

        Integer maxKey(int val) {
            TreeSet<Integer> set = byVal.get(val);
            return (set == null || set.isEmpty()) ? null : set.last();
        }
    }

    public static void main(String[] args) {
        KVStore kv = new KVStore();
        kv.update(1, 1);
        kv.update(2, 1);
        System.out.println(kv.maxKey(1)); // 2
    }
}
