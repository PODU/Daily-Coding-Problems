// Day 1024: Reverse all 32 bits of a 32-bit integer.
// Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.
public class Solution {
    static int reverseBits(int n) {
        n = (n >>> 16) | (n << 16);
        n = ((n & 0xFF00FF00) >>> 8) | ((n & 0x00FF00FF) << 8);
        n = ((n & 0xF0F0F0F0) >>> 4) | ((n & 0x0F0F0F0F) << 4);
        n = ((n & 0xCCCCCCCC) >>> 2) | ((n & 0x33333333) << 2);
        n = ((n & 0xAAAAAAAA) >>> 1) | ((n & 0x55555555) << 1);
        return n;
    }

    static String grouped(int n) {
        StringBuilder s = new StringBuilder();
        for (int i = 31; i >= 0; --i) {
            s.append((n >>> i) & 1);
            if (i % 4 == 0 && i != 0) s.append(' ');
        }
        return s.toString();
    }

    public static void main(String[] args) {
        int x = 0xF0F0F0F0;
        System.out.println(grouped(reverseBits(x)));
    }
}
