// Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
// Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.
public class Solution {
    static long maxNonAdjacent(int[] nums) {
        long incl = 0, excl = 0;
        for (int n : nums) {
            long newIncl = excl + n;
            long newExcl = Math.max(incl, excl);
            incl = newIncl;
            excl = newExcl;
        }
        return Math.max(incl, excl);
    }

    public static void main(String[] args) {
        System.out.println(maxNonAdjacent(new int[]{2, 4, 6, 2, 5})); // 13
        System.out.println(maxNonAdjacent(new int[]{5, 1, 1, 5}));    // 10
    }
}
