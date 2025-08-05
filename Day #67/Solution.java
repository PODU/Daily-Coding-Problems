// LFU cache: key->(value,freq), freq->LinkedHashSet of keys (LRU within freq), track minFreq. O(1) get/set.
import java.util.*;

public class Solution {
    static class LFUCache {
        private final int cap;
        private int minFreq;
        private final Map<Integer,Integer> values = new HashMap<>();
        private final Map<Integer,Integer> freqs = new HashMap<>();
        private final Map<Integer,LinkedHashSet<Integer>> freqList = new HashMap<>();

        LFUCache(int capacity) { this.cap = capacity; this.minFreq = 0; }

        private void touch(int key) {
            int f = freqs.get(key);
            LinkedHashSet<Integer> s = freqList.get(f);
            s.remove(key);
            if (s.isEmpty()) {
                freqList.remove(f);
                if (minFreq == f) minFreq = f + 1;
            }
            int nf = f + 1;
            freqs.put(key, nf);
            freqList.computeIfAbsent(nf, k -> new LinkedHashSet<>()).add(key);
        }

        Integer get(int key) {
            if (!values.containsKey(key)) return null;
            touch(key);
            return values.get(key);
        }

        void set(int key, int value) {
            if (cap <= 0) return;
            if (values.containsKey(key)) {
                values.put(key, value);
                touch(key);
                return;
            }
            if (values.size() >= cap) {
                LinkedHashSet<Integer> s = freqList.get(minFreq);
                int evict = s.iterator().next(); // least recently used at min freq
                s.remove(evict);
                if (s.isEmpty()) freqList.remove(minFreq);
                values.remove(evict);
                freqs.remove(evict);
            }
            values.put(key, value);
            freqs.put(key, 1);
            freqList.computeIfAbsent(1, k -> new LinkedHashSet<>()).add(key);
            minFreq = 1;
        }
    }

    public static void main(String[] args) {
        LFUCache c = new LFUCache(2);
        c.set(1,1);
        c.set(2,2);
        System.out.println(c.get(1));   // 1
        c.set(3,3);                     // evicts key 2
        System.out.println(c.get(2));   // null
        System.out.println(c.get(3));   // 3
        c.set(4,4);                     // evicts key 1
        System.out.println(c.get(1));   // null
        System.out.println(c.get(3));   // 3
        System.out.println(c.get(4));   // 4
    }
}
