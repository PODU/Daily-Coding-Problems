// Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a seen[] array.
// O(n) time, O(n) space.
public class Solution {
    static int findDuplicate(int[] nums) {
        boolean[] seen = new boolean[nums.length + 1];
        for (int x : nums) {
            if (seen[x]) return x;
            seen[x] = true;
        }
        return -1; // no duplicate (won't happen per problem constraints)
    }

    public static void main(String[] args) {
        int[] nums = {1, 3, 4, 2, 2};
        System.out.println(findDuplicate(nums)); // expected 2
    }
}
