// Space-efficient bit array packing 32 bits per word.
// init(size), set(i,val), get(i): each O(1); space O(size/32) words.
public class Solution {
    static class BitArray {
        int[] words;
        int n;
        void init(int size) {
            n = size;
            words = new int[(size + 31) / 32];
        }
        void set(int i, int val) {
            if (val != 0) words[i >> 5] |= (1 << (i & 31));
            else words[i >> 5] &= ~(1 << (i & 31));
        }
        int get(int i) {
            return (words[i >> 5] >>> (i & 31)) & 1;
        }
    }

    public static void main(String[] args) {
        BitArray b = new BitArray();
        b.init(10);
        b.set(1, 1);
        b.set(4, 1);
        b.set(4, 0);
        b.set(9, 1);
        System.out.println(b.get(1) + " " + b.get(4) + " " + b.get(9) + " " + b.get(0));
        // expected: 1 0 1 0
    }
}
