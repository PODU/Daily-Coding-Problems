// Day 1431: Majority element (appears > floor(n/2)).
// Approach: Boyer-Moore voting. Time: O(n), Space: O(1).
public class Solution {
    static int majorityElement(int[] nums) {
        int count = 0, candidate = 0;
        for (int x : nums) {
            if (count == 0) candidate = x;
            count += (x == candidate) ? 1 : -1;
        }
        return candidate;
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 1, 1, 3, 4, 0};
        System.out.println(majorityElement(nums)); // 1
    }
}
