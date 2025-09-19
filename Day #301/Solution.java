// Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
// add/check O(k); space O(m) bits.
public class Solution {
    static class BloomFilter {
        boolean[] bits; int k;
        BloomFilter(int m, int k) { bits = new boolean[m]; this.k = k; }
        int h(String s, int i) {
            long h1 = s.hashCode();
            long h2 = (s + "#salt").hashCode();
            long idx = (h1 + (long) i * h2) % bits.length;
            if (idx < 0) idx += bits.length;
            return (int) idx;
        }
        void add(String v) { for (int i = 0; i < k; i++) bits[h(v, i)] = true; }
        boolean check(String v) {
            for (int i = 0; i < k; i++) if (!bits[h(v, i)]) return false;
            return true;
        }
    }
    public static void main(String[] a) {
        BloomFilter bf = new BloomFilter(1000, 4);
        bf.add("apple"); bf.add("banana");
        System.out.println(bf.check("apple"));
        System.out.println(bf.check("banana"));
        System.out.println(bf.check("cherry"));
    }
}
