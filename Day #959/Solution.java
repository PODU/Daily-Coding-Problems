// Day 959: total number of set bits over all integers in [1, N].
// Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).
public class Solution {
    static long countSetBits(long n) {
        long total = 0;
        for (int i = 0; (1L << i) <= n; i++) {
            long cycle = 1L << (i + 1);
            long half = cycle >> 1;
            total += (n + 1) / cycle * half;
            long rem = (n + 1) % cycle;
            total += Math.max(0L, rem - half);
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countSetBits(5)); // 7
    }
}
