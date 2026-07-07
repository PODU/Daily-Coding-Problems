// Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
// Time: O(N), Space: O(1).
public class Solution {
    static long maxNonAdjacent(int[] a) {
        long incl = 0, excl = 0;
        for (int n : a) {
            long ni = excl + n;
            long ne = Math.max(incl, excl);
            incl = ni; excl = ne;
        }
        return Math.max(incl, excl);
    }

    public static void main(String[] args) {
        System.out.println(maxNonAdjacent(new int[]{2,4,6,2,5}));
        System.out.println(maxNonAdjacent(new int[]{5,1,1,5}));
    }
}
