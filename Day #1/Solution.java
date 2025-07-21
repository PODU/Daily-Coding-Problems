// Two-sum existence: one pass with a hash set of complements.
// Time: O(n), Space: O(n).
import java.util.HashSet;
import java.util.Set;

public class Solution {
    static boolean twoSum(int[] nums, int k) {
        Set<Integer> seen = new HashSet<>();
        for (int x : nums) {
            if (seen.contains(k - x)) return true;
            seen.add(x);
        }
        return false;
    }

    public static void main(String[] args) {
        int[] nums = {10, 15, 3, 7};
        int k = 17;
        System.out.println(twoSum(nums, k));
    }
}
