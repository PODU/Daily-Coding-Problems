// Day 1579: Maximum circular subarray sum (empty allowed -> 0).
// ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).
public class Solution {
    static long maxCircular(int[] a) {
        long total = 0, maxEnd = 0, maxSum = 0;
        long minEnd = 0, minSum = Long.MAX_VALUE;
        for (int x : a) {
            total += x;
            maxEnd = Math.max(x, maxEnd + x);
            maxSum = Math.max(maxSum, maxEnd);
            minEnd = Math.min(x, minEnd + x);
            minSum = Math.min(minSum, minEnd);
        }
        return Math.max(maxSum, total - minSum);
    }

    public static void main(String[] args) {
        System.out.println(maxCircular(new int[]{8, -1, 3, 4})); // 15
        System.out.println(maxCircular(new int[]{-4, 5, 1, 0})); // 6
    }
}
