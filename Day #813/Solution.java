// Non-decreasing with at most 1 modification: single pass counting violations,
// greedily lower nums[i-1] or raise nums[i]. Time O(n), Space O(1).
public class Solution {
    static boolean canBeNonDecreasing(int[] nums) {
        int count = 0;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i - 1] > nums[i]) {
                if (++count > 1) return false;
                if (i < 2 || nums[i - 2] <= nums[i]) nums[i - 1] = nums[i];
                else nums[i] = nums[i - 1];
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canBeNonDecreasing(new int[]{10, 5, 7}));
        System.out.println(canBeNonDecreasing(new int[]{10, 5, 1}));
    }
}
