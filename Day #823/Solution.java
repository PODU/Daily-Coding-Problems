// Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
// Time: O(1) per op, Space: O(n/32 words).
public class Solution {
    static class BitArray {
        int[] words;
        BitArray(int size) { words = new int[(size + 31) >> 5]; }
        void set(int i, int val) {
            if (val != 0) words[i >> 5] |= (1 << (i & 31));
            else          words[i >> 5] &= ~(1 << (i & 31));
        }
        int get(int i) { return (words[i >> 5] >> (i & 31)) & 1; }
    }

    public static void main(String[] args) {
        BitArray ba = new BitArray(16);
        ba.set(0, 1);
        ba.set(5, 1);
        ba.set(15, 1);
        System.out.println("get(0)=" + ba.get(0));
        System.out.println("get(1)=" + ba.get(1));
        System.out.println("get(5)=" + ba.get(5));
        System.out.println("get(15)=" + ba.get(15));
    }
}
