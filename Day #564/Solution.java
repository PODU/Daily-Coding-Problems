// Day 564: Largest sum of non-adjacent numbers (empty subset allowed -> answer >= 0).
// DP tracking incl/excl running maxes. Time O(n), Space O(1).
public class Solution {
    static long largestNonAdjacent(long[] nums) {
        long incl = 0, excl = 0; // best sums including / excluding previous element
        for (long x : nums) {
            long newIncl = excl + x;
            excl = Math.max(incl, excl);
            incl = newIncl;
        }
        return Math.max(Math.max(incl, excl), 0L);
    }

    public static void main(String[] args) {
        System.out.println(largestNonAdjacent(new long[]{2, 4, 6, 2, 5}));
        System.out.println(largestNonAdjacent(new long[]{5, 1, 1, 5}));
    }
}
