// Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
public class Solution {
    static long countBits(long N) {
        long total = 0;
        for (long i = 0; (1L << i) <= N; i++) {
            long blk = 1L << (i + 1);
            long full = (N + 1) / blk * (1L << i);
            long rem = Math.max(0L, (N + 1) % blk - (1L << i));
            total += full + rem;
        }
        return total;
    }
    public static void main(String[] a) {
        System.out.println(countBits(5)); // 7
    }
}
