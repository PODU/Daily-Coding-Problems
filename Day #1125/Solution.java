// Day 1125 - Contiguous sublist summing to K
// Prefix sums + hash map (handles negatives) to find a range with sum == K in
// one pass. Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> subarraySum(int[] nums, int k) {
        Map<Long, Integer> seen = new HashMap<>();
        seen.put(0L, -1);
        long total = 0;
        for (int j = 0; j < nums.length; j++) {
            total += nums[j];
            if (seen.containsKey(total - k)) {
                int i = seen.get(total - k);
                List<Integer> res = new ArrayList<>();
                for (int t = i + 1; t <= j; t++) res.add(nums[t]);
                return res;
            }
            seen.putIfAbsent(total, j);
        }
        return null;
    }

    public static void main(String[] args) {
        System.out.println(subarraySum(new int[]{1, 2, 3, 4, 5}, 9)); // [2, 3, 4]
    }
}
