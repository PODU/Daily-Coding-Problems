// Max of two ints without if/else/branch/ternary/comparison.
// Use sign bit of (a-b) via 64-bit diff to avoid overflow. O(1) time, O(1) space.
public class Solution {
    static long maxOf(long a, long b) {
        long d = a - b;                  // safe in 64-bit for 32-bit inputs
        long sign = (d >> 63) & 1L;      // 1 if a<b, else 0
        return a - sign * d;             // a - (a-b) = b when a<b, else a
    }

    public static void main(String[] args) {
        System.out.println("max(3, 7) = " + maxOf(3, 7));
        System.out.println("max(10, 2) = " + maxOf(10, 2));
    }
}
