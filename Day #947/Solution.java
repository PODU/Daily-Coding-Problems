// Day 947: smallest sparse number (no two adjacent set bits) >= N.
// Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).
public class Solution {
    static long smallestSparse(long n) {
        while ((n & (n >> 1)) != 0L) {
            long m = n & (n >> 1);
            int p = Long.numberOfTrailingZeros(m);
            long step = 1L << (p + 1);
            n = (n + step) & ~(step - 1);
        }
        return n;
    }

    public static void main(String[] args) {
        System.out.println(smallestSparse(21)); // 21
        System.out.println(smallestSparse(22)); // 32
    }
}
