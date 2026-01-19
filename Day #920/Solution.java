// Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
// add/check are O(k); space O(m) bits. check has false positives, no false negatives.
public class Solution {
    static class BloomFilter {
        static final int SIZE = 1000;
        static final int K = 3;
        boolean[] bits = new boolean[SIZE];

        private int[] baseHashes(String s) {
            int h1 = s.hashCode();
            int h2 = (s + "salt").hashCode();
            return new int[]{h1, h2};
        }
        private int idx(int h1, int h2, int i) {
            return Math.floorMod(h1 + i * h2, SIZE);
        }
        void add(String v) {
            int[] h = baseHashes(v);
            for (int i = 0; i < K; i++) bits[idx(h[0], h[1], i)] = true;
        }
        boolean check(String v) {
            int[] h = baseHashes(v);
            for (int i = 0; i < K; i++) if (!bits[idx(h[0], h[1], i)]) return false;
            return true;
        }
    }

    public static void main(String[] args) {
        BloomFilter bf = new BloomFilter();
        String[] added = {"apple", "banana", "cherry"};
        for (String s : added) bf.add(s);

        System.out.println("Added values (expect all true):");
        for (String s : added)
            System.out.println("  check(" + s + ") = " + bf.check(s));

        System.out.println("Non-added values (expect mostly false):");
        for (String s : new String[]{"date", "elderberry", "fig", "grape"})
            System.out.println("  check(" + s + ") = " + bf.check(s));
    }
}
