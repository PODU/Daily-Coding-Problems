// Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
// Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
// Time O(N), Space O(1).
public class Solution {
    static long maxCircularSubarray(int[] a) {
        long total = 0, curMax = 0, bestMax = 0, curMin = 0, bestMin = 0;
        for (int x : a) {
            total += x;
            curMax = Math.max(x, curMax + x); bestMax = Math.max(bestMax, curMax);
            curMin = Math.min(x, curMin + x); bestMin = Math.min(bestMin, curMin);
        }
        long wrap = total - bestMin;
        return Math.max(0, Math.max(bestMax, wrap));
    }

    public static void main(String[] args) {
        System.out.println(maxCircularSubarray(new int[]{8, -1, 3, 4})); // 15
        System.out.println(maxCircularSubarray(new int[]{-4, 5, 1, 0})); // 6
    }
}
