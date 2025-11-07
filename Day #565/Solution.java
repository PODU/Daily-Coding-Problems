// Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
public class Solution {
    static boolean canJump(int[] nums) {
        int farthest = 0;
        for (int i = 0; i < nums.length; i++) {
            if (i > farthest) return false;
            farthest = Math.max(farthest, i + nums[i]);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canJump(new int[]{2, 0, 1, 0}) ? "True" : "False");
        System.out.println(canJump(new int[]{1, 1, 0, 1}) ? "True" : "False");
    }
}
