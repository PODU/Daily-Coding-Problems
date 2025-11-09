// Day 574: Space-efficient bit array backed by 64-bit longs.
// set/get are O(1); storage is ceil(size/64) words.
public class Solution {
    static class BitArray {
        private final long[] words;
        private final int n;
        BitArray(int size) {
            this.n = size;
            this.words = new long[(size + 63) >> 6];
        }
        void set(int i, int val) {
            if (i < 0 || i >= n) throw new IndexOutOfBoundsException();
            if (val != 0) words[i >> 6] |=  (1L << (i & 63));
            else          words[i >> 6] &= ~(1L << (i & 63));
        }
        int get(int i) {
            if (i < 0 || i >= n) throw new IndexOutOfBoundsException();
            return (int) ((words[i >> 6] >>> (i & 63)) & 1L);
        }
    }

    public static void main(String[] args) {
        BitArray b = new BitArray(8);
        b.set(0, 1);
        b.set(3, 1);
        System.out.println("get(0) = " + b.get(0)); // 1
        System.out.println("get(1) = " + b.get(1)); // 0
        System.out.println("get(3) = " + b.get(3)); // 1
        b.set(3, 0);
        System.out.println("get(3) = " + b.get(3)); // 0
    }
}
