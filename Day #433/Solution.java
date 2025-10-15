// Day 433: Next larger integer with the same number of set bits (Gosper's hack).
// c = n & -n; r = n + c; next = (((r ^ n) >> 2) / c) | r. O(1) time, O(1) space.
public class Solution {
    static long nextSameBits(long n) {
        if (n <= 0) return n;
        long c = n & (-n);
        long r = n + c;
        return (((r ^ n) >> 2) / c) | r;
    }

    public static void main(String[] args) {
        long n = 6, m = nextSameBits(n);
        System.out.println("Input: " + n + " (" + Long.toBinaryString(n) + " in binary)");
        System.out.println("Next: " + m + " (" + Long.toBinaryString(m) + " in binary)");
    }
}
