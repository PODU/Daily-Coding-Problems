// Find the duplicate in array of length n+1 with values in {1..n}.
// Floyd's tortoise-and-hare cycle detection. Time O(n), Space O(1).
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
        int[] nums = {1, 2, 3, 4, 2}; // n = 4
        System.out.println(findDuplicate(nums)); // expected: 2
    }
}
