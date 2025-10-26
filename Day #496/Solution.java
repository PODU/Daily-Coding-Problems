// Day 496: Total set bits across 1..N.
// For each bit position, count how many numbers in [0,N] have it set using the
// periodic pattern. O(log N) time, O(1) space.
public class Solution {
    static long countSetBits(long n) {
        long total = 0;
        for (long bit = 1; bit <= n; bit <<= 1) {
            long full = n + 1;          // count of integers in [0, n]
            long cycle = bit << 1;      // period for this bit
            total += (full / cycle) * bit;
            long rem = full % cycle;
            total += Math.max(0L, rem - bit);
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countSetBits(5));  // 7
        System.out.println(countSetBits(16)); // 33
    }
}
