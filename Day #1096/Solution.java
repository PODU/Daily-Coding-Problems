// Day 1096: LFU Cache with O(1) get/set.
// Approach: key->value/freq maps + freq->LinkedHashSet(LRU) + minFreq pointer.
// Time: O(1) per op. Space: O(n).
import java.util.*;

public class Solution {
    static class LFUCache {
        int cap, minFreq;
        Map<Integer,Integer> vals = new HashMap<>();
        Map<Integer,Integer> freqs = new HashMap<>();
        Map<Integer,LinkedHashSet<Integer>> lists = new HashMap<>();

        LFUCache(int n){ cap = n; minFreq = 0; }

        boolean has(int key){ return vals.containsKey(key); }

        Integer get(int key){
            if (!vals.containsKey(key)) return null;
            touch(key);
            return vals.get(key);
        }

        void touch(int key){
            int f = freqs.get(key);
            lists.get(f).remove(key);
            if (lists.get(f).isEmpty()){
                lists.remove(f);
                if (minFreq == f) minFreq++;
            }
            int nf = f + 1;
            freqs.put(key, nf);
            lists.computeIfAbsent(nf, k -> new LinkedHashSet<>()).add(key);
        }

        void set(int key, int value){
            if (cap <= 0) return;
            if (vals.containsKey(key)){ vals.put(key, value); touch(key); return; }
            if (vals.size() >= cap){
                LinkedHashSet<Integer> ls = lists.get(minFreq);
                int evict = ls.iterator().next(); // least recently used at this freq
                ls.remove(evict);
                if (ls.isEmpty()) lists.remove(minFreq);
                vals.remove(evict); freqs.remove(evict);
            }
            vals.put(key, value); freqs.put(key, 1);
            lists.computeIfAbsent(1, k -> new LinkedHashSet<>()).add(key);
            minFreq = 1;
        }
    }

    public static void main(String[] args){
        LFUCache c = new LFUCache(2);
        c.set(1,1); c.set(2,2);
        System.out.println(c.get(1));            // 1
        c.set(3,3);                              // evicts key 2
        System.out.println(c.has(2) ? "2" : "null"); // null
        System.out.println(c.get(3));            // 3
    }
}
