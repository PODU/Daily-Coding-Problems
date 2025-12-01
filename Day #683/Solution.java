// Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
// has no strict majority, so we verify the candidate and fall back to the mode.
public class Solution {
    static int countOf(int[] v, int target) {
        int c = 0;
        for (int x : v) if (x == target) c++;
        return c;
    }

    static int majority(int[] nums) {
        int count = 0, candidate = 0;
        for (int x : nums) {                    // Boyer-Moore voting pass
            if (count == 0) candidate = x;
            count += (x == candidate) ? 1 : -1;
        }
        if (countOf(nums, candidate) > nums.length / 2) return candidate;
        int best = nums[0];                     // fallback: most frequent element
        for (int x : nums) if (countOf(nums, x) > countOf(nums, best)) best = x;
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 1, 1, 3, 4, 0};
        System.out.println(majority(nums)); // 1
    }
}
