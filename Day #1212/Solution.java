// Day 1212: Space-efficient bit array backed by 64-bit words.
// Pack bits into words; set/get use word index and bit offset. Time O(1) per op, Space O(size/64).
public class Solution {
    static class BitArray {
        long[] words;
        BitArray(int size) { words = new long[(size + 63) / 64]; }
        void set(int i, int val) {
            if (val != 0) words[i >> 6] |= (1L << (i & 63));
            else words[i >> 6] &= ~(1L << (i & 63));
        }
        int get(int i) { return (int) ((words[i >> 6] >> (i & 63)) & 1L); }
    }

    public static void main(String[] args) {
        BitArray b = new BitArray(10);
        b.set(2, 1); b.set(7, 1); b.set(2, 0);
        System.out.println(b.get(2) + " " + b.get(7) + " " + b.get(0)); // 0 1 0
    }
}
