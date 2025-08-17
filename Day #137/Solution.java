// Space-efficient bit array: pack bits into 64-bit longs. set/get O(1), space O(size/64).

public class Solution {
    static class BitArray {
        long[] words;
        void init(int size) { words = new long[(size + 63) / 64]; }
        void set(int i, int val) {
            if (val != 0) words[i >> 6] |= (1L << (i & 63));
            else          words[i >> 6] &= ~(1L << (i & 63));
        }
        int get(int i) { return (int) ((words[i >> 6] >>> (i & 63)) & 1L); }
    }

    public static void main(String[] args) {
        BitArray b = new BitArray(); b.init(10);
        b.set(2, 1); b.set(7, 1); b.set(7, 0); b.set(9, 1);
        System.out.println("" + b.get(2) + b.get(7) + b.get(9) + b.get(0)); // 1010
    }
}
