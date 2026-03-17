// Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.
public class Solution {
    static long countSetBits(long n) {
        long total = 0;
        for (long p = 2; p <= 2 * n; p <<= 1) {
            long full = ((n + 1) / p) * (p / 2);
            long rem = Math.max(0L, (n + 1) % p - p / 2);
            total += full + rem;
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countSetBits(5));
    }
}
