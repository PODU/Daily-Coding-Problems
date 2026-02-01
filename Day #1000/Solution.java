// Day 1000: Minimum of a rotated sorted array (no duplicates).
// Binary search comparing mid with the right end. O(log N) time, O(1) space.
public class Solution {
    static int findMin(int[] nums) {
        int lo = 0, hi = nums.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (nums[mid] > nums[hi]) lo = mid + 1;
            else hi = mid;
        }
        return nums[lo];
    }

    public static void main(String[] args) {
        System.out.println(findMin(new int[]{5, 7, 10, 3, 4})); // 3
    }
}
