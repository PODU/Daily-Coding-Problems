// Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
public class Solution {
    static boolean checkPossibility(int[] nums) {
        int cnt = 0;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] < nums[i - 1]) {
                if (++cnt > 1) return false;
                if (i < 2 || nums[i - 2] <= nums[i]) nums[i - 1] = nums[i];
                else nums[i] = nums[i - 1];
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(checkPossibility(new int[]{10, 5, 7}));
        System.out.println(checkPossibility(new int[]{10, 5, 1}));
    }
}
