// Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.
public class Solution {
    static long maxSubarraySum(int[] a) {
        long best = 0, cur = 0;
        for (int x : a) {
            cur = Math.max(0, cur + x);
            best = Math.max(best, cur);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(maxSubarraySum(new int[]{34, -50, 42, 14, -5, 86})); // 137
        System.out.println(maxSubarraySum(new int[]{-5, -1, -8, -9}));          // 0
    }
}
