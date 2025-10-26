// Day 494: Maximum circular subarray sum (empty allowed => 0).
// max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
public class Solution {
    static long maxCircularSubarray(long[] a) {
        long total = 0;
        long maxK = Long.MIN_VALUE, curMax = 0;
        long minK = Long.MAX_VALUE, curMin = 0;
        for (long x : a) {
            total += x;
            curMax = Math.max(x, curMax + x);
            maxK = Math.max(maxK, curMax);
            curMin = Math.min(x, curMin + x);
            minK = Math.min(minK, curMin);
        }
        if (maxK < 0) return 0; // all negative -> empty subarray
        return Math.max(maxK, total - minK);
    }

    public static void main(String[] args) {
        System.out.println(maxCircularSubarray(new long[]{8, -1, 3, 4})); // 15
        System.out.println(maxCircularSubarray(new long[]{-4, 5, 1, 0})); // 6
    }
}
