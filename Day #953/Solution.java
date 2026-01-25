// Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
// incl/excl DP. Time O(n), Space O(1).
public class Solution {
    static long maxNonAdjacent(long[] a) {
        long incl = 0, excl = 0;
        for (long x : a) {
            long ni = excl + x;
            long ne = Math.max(incl, excl);
            incl = ni; excl = ne;
        }
        return Math.max(incl, excl);
    }

    public static void main(String[] args) {
        System.out.println(maxNonAdjacent(new long[]{2, 4, 6, 2, 5})); // 13
        System.out.println(maxNonAdjacent(new long[]{5, 1, 1, 5}));    // 10
    }
}
