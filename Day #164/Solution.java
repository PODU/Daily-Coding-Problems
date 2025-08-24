// Day 164: Find duplicate via Floyd's cycle detection (values as next-pointers).
// O(n) time, O(1) extra space (beats the O(n)-space requirement).
public class Solution {
    static int findDuplicate(int[] nums) {
        int slow = nums[0], fast = nums[0];
        do {
            slow = nums[slow];
            fast = nums[nums[fast]];
        } while (slow != fast);
        slow = nums[0];
        while (slow != fast) {
            slow = nums[slow];
            fast = nums[fast];
        }
        return slow;
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3, 4, 2}; // n = 4, length n+1
        System.out.println(findDuplicate(nums)); // 2
    }
}
