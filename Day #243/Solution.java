// Split array into k parts minimizing max partition sum.
// Binary search on answer in [max, sum], greedy feasibility check. O(n log(sum)).
public class Solution {
    static boolean canSplit(int[] nums, int k, long cap) {
        int parts = 1;
        long cur = 0;
        for (int x : nums) {
            if (cur + x > cap) { parts++; cur = x; }
            else cur += x;
        }
        return parts <= k;
    }

    static long splitArray(int[] nums, int k) {
        long lo = 0, hi = 0;
        for (int x : nums) { lo = Math.max(lo, x); hi += x; }
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (canSplit(nums, k, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    public static void main(String[] args) {
        int[] nums = {5, 1, 2, 7, 3, 4};
        System.out.println(splitArray(nums, 3));
    }
}
