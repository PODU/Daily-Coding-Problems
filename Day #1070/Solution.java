// Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static int longestAtMost2Distinct(int[] nums) {
        Map<Integer,Integer> cnt = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < nums.length; right++) {
            cnt.merge(nums[right], 1, Integer::sum);
            while (cnt.size() > 2) {
                cnt.merge(nums[left], -1, Integer::sum);
                if (cnt.get(nums[left]) == 0) cnt.remove(nums[left]);
                left++;
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {2,1,2,3,3,1,3,5};
        System.out.println(longestAtMost2Distinct(nums));
    }
}
