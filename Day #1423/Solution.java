// Day 1423: can you reach the end of the array (each value = max steps forward)?
// Approach: greedy, track farthest reachable index. O(n) time, O(1) space.
public class Solution {
    static boolean canReachEnd(int[] nums) {
        int farthest = 0;
        for (int i = 0; i < nums.length; i++) {
            if (i > farthest) return false;
            farthest = Math.max(farthest, i + nums[i]);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canReachEnd(new int[]{1, 3, 1, 2, 0, 1}) ? "true" : "false"); // true
        System.out.println(canReachEnd(new int[]{1, 2, 1, 0, 0}) ? "true" : "false");    // false
    }
}
