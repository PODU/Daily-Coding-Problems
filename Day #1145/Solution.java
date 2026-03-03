// Day 1145: Bloom filter - fixed-size bit array, k hashes.
// add/check O(k); check has false positives but never false negatives.
import java.util.BitSet;

public class Solution {
    static class BloomFilter {
        BitSet bits;
        int m, k;
        BloomFilter(int m, int k) { this.m = m; this.k = k; bits = new BitSet(m); }
        int hashI(String s, int i) {
            long h = 1469598103934665603L ^ (i + 1);
            for (int j = 0; j < s.length(); j++) { h ^= s.charAt(j); h *= 1099511628211L; }
            return (int) (Math.floorMod(h, m));
        }
        void add(String v) { for (int i = 0; i < k; i++) bits.set(hashI(v, i)); }
        boolean check(String v) {
            for (int i = 0; i < k; i++) if (!bits.get(hashI(v, i))) return false;
            return true;
        }
    }

    public static void main(String[] args) {
        BloomFilter bf = new BloomFilter(1000, 4);
        bf.add("apple"); bf.add("banana");
        System.out.println(bf.check("apple") + " " + bf.check("banana")
                + " " + bf.check("cherry")); // true true false (likely)
    }
}
