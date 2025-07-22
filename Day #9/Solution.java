// Max sum of non-adjacent numbers: track best-including vs best-excluding current.
// Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
public class Solution {
    static long maxNonAdjacent(int[] nums) {
        long incl = 0, excl = 0;
        for (int n : nums) {
            long ni = excl + n;
            long ne = Math.max(incl, excl);
            incl = ni;
            excl = ne;
        }
        return Math.max(incl, excl);
    }

    public static void main(String[] args) {
        System.out.println(maxNonAdjacent(new int[]{2, 4, 6, 2, 5}));
    }
}
