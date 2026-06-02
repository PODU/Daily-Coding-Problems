// Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
// For each bit i: full cycles contribute 2^i ones each, plus remainder.
public class Solution {
    static long countSetBits(long n) {
        long total = 0;
        for (long i = 0; (1L << i) <= n; i++) {
            long block = 1L << (i + 1);
            long ones = (n + 1) / block * (1L << i);
            long rem = (n + 1) % block - (1L << i);
            if (rem > 0) ones += rem;
            total += ones;
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println("N=5  -> " + countSetBits(5));   // 7
        System.out.println("N=16 -> " + countSetBits(16));  // 33
    }
}
