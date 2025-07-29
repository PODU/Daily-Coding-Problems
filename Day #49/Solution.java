// Day 49: Maximum contiguous subarray sum (Kadane), empty subarray allowed.
// Time: O(n), Space: O(1).
public class Solution {
    static long maxSubarray(int[] a) {
        long best = 0, cur = 0; // empty subarray => 0
        for (int x : a) {
            cur = Math.max(0, cur + x);
            best = Math.max(best, cur);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(maxSubarray(new int[]{34, -50, 42, 14, -5, 86}));
        System.out.println(maxSubarray(new int[]{-5, -1, -8, -9}));
    }
}
