// Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
// Bump jumps when current window ends. Time O(n), Space O(1).
public class Solution {
    static int minJumps(int[] nums) {
        int n = nums.length;
        if (n <= 1) return 0;
        int jumps = 0, curEnd = 0, farthest = 0;
        for (int i = 0; i < n - 1; i++) {
            farthest = Math.max(farthest, i + nums[i]);
            if (i == curEnd) { jumps++; curEnd = farthest; }
        }
        return jumps;
    }

    public static void main(String[] args) {
        int[] nums = {6, 2, 4, 0, 5, 1, 1, 4, 2, 9};
        System.out.println(minJumps(nums));
    }
}
