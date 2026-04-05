// Day 1303: Next larger integer with the same number of set bits (snoob).
// Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.
public class Solution {
    static long nextSameBits(long n) {
        long c = n & (-n);            // lowest set bit
        long r = n + c;               // ripple carry
        long ones = ((n ^ r) >> 2) / c; // moved bits, shifted down
        return r | ones;
    }

    public static void main(String[] args) {
        System.out.println(nextSameBits(6)); // 9
    }
}
