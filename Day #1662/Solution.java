// O(1) LFU cache: key->node map, freq->list(ordered by recency), minFreq pointer.
// get/set O(1); Space O(capacity). Evict least-freq, tie -> least-recently-used.
import java.util.*;
public class Solution {
    static class LFU {
        int cap, minFreq = 0;
        Map<Integer,Integer> vals = new HashMap<>(), freqs = new HashMap<>();
        Map<Integer,LinkedHashSet<Integer>> lists = new HashMap<>(); // freq -> keys (insertion order = LRU..MRU)
        LFU(int c) { cap = c; }
        void touch(int key) {
            int f = freqs.get(key);
            lists.get(f).remove(key);
            if (lists.get(f).isEmpty()) { lists.remove(f); if (minFreq == f) minFreq++; }
            int nf = f + 1; freqs.put(key, nf);
            lists.computeIfAbsent(nf, k -> new LinkedHashSet<>()).add(key);
        }
        Integer get(int key) {
            if (!vals.containsKey(key)) return null;
            touch(key);
            return vals.get(key);
        }
        void set(int key, int val) {
            if (cap <= 0) return;
            if (vals.containsKey(key)) { vals.put(key, val); touch(key); return; }
            if (vals.size() >= cap) {
                Iterator<Integer> it = lists.get(minFreq).iterator();
                int lru = it.next(); it.remove();
                if (lists.get(minFreq).isEmpty()) lists.remove(minFreq);
                vals.remove(lru); freqs.remove(lru);
            }
            vals.put(key, val); freqs.put(key, 1);
            lists.computeIfAbsent(1, k -> new LinkedHashSet<>()).add(key);
            minFreq = 1;
        }
    }
    static void show(Integer v) { System.out.println(v == null ? "null" : v.toString()); }
    public static void main(String[] args) {
        LFU c = new LFU(2);
        c.set(1,1); c.set(2,2);
        show(c.get(1));
        c.set(3,3);
        show(c.get(2));
        show(c.get(3));
        c.set(4,4);
        show(c.get(1));
        show(c.get(3));
        show(c.get(4));
    }
}
