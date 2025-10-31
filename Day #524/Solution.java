// Kadane's algorithm: max contiguous subarray sum, empty subarray allowed (>=0).
// Time O(n), Space O(1).
public class Solution {
    static long maxSubarraySum(long[] a) {
        long best = 0, cur = 0; // empty subarray allowed -> min 0
        for (long x : a) {
            cur = Math.max(0, cur + x);
            best = Math.max(best, cur);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(maxSubarraySum(new long[]{34, -50, 42, 14, -5, 86}));
        System.out.println(maxSubarraySum(new long[]{-5, -1, -8, -9}));
    }
}
