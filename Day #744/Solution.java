// O(1) LFU cache. Maps key->value/freq; each freq holds a LinkedHashSet of keys
// (insertion order = recency) so eviction picks least-frequent, then least-recent.
// Time: O(1) per get/set, Space: O(capacity).
import java.util.*;

public class Solution {
    static class LFUCache {
        int cap, minFreq;
        Map<Integer,Integer> val = new HashMap<>();
        Map<Integer,Integer> freq = new HashMap<>();
        Map<Integer,LinkedHashSet<Integer>> buckets = new HashMap<>();

        LFUCache(int capacity){ cap = capacity; }

        void bump(int key){
            int f = freq.get(key);
            LinkedHashSet<Integer> b = buckets.get(f);
            b.remove(key);
            if(b.isEmpty()){
                buckets.remove(f);
                if(minFreq == f) minFreq++;
            }
            freq.put(key, f + 1);
            buckets.computeIfAbsent(f + 1, k -> new LinkedHashSet<>()).add(key);
        }

        Integer get(int key){
            if(!val.containsKey(key)) return null;
            bump(key);
            return val.get(key);
        }

        void set(int key, int value){
            if(cap <= 0) return;
            if(val.containsKey(key)){
                val.put(key, value);
                bump(key);
                return;
            }
            if(val.size() >= cap){
                LinkedHashSet<Integer> b = buckets.get(minFreq);
                int evict = b.iterator().next();
                b.remove(evict);
                val.remove(evict);
                freq.remove(evict);
            }
            val.put(key, value);
            freq.put(key, 1);
            buckets.computeIfAbsent(1, k -> new LinkedHashSet<>()).add(key);
            minFreq = 1;
        }
    }

    public static void main(String[] args){
        LFUCache c = new LFUCache(2);
        c.set(1, 1);
        c.set(2, 2);
        System.out.println(c.get(1)); // 1
        c.set(3, 3);                   // evicts key 2
        System.out.println(c.get(2)); // null
        System.out.println(c.get(3)); // 3
        c.set(4, 4);                   // evicts key 1
        System.out.println(c.get(1)); // null
        System.out.println(c.get(3)); // 3
        System.out.println(c.get(4)); // 4
    }
}
