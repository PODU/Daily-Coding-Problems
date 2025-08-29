// Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
// ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
public class Solution {
    static long maxCircularSum(int[] a) {
        long total = 0, maxK = Long.MIN_VALUE, curMax = 0, minK = Long.MAX_VALUE, curMin = 0;
        for (int x : a) {
            total += x;
            curMax = Math.max(x, curMax + x); maxK = Math.max(maxK, curMax);
            curMin = Math.min(x, curMin + x); minK = Math.min(minK, curMin);
        }
        long ans = Math.max(maxK, total - minK);
        return Math.max(0L, ans);
    }

    public static void main(String[] args) {
        System.out.println(maxCircularSum(new int[]{8, -1, 3, 4}));
        System.out.println(maxCircularSum(new int[]{-4, 5, 1, 0}));
    }
}
