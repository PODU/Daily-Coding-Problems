// Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
// prefix p look for p-K seen earlier; earliest-ending match. O(n) time.
import java.util.*;

public class Solution {
    static int[] subarraySum(int[] nums, int k) {
        Map<Long, Integer> first = new HashMap<>();
        first.put(0L, -1);
        long prefix = 0;
        for (int j = 0; j < nums.length; j++) {
            prefix += nums[j];
            Integer i = first.get(prefix - k);
            if (i != null) return Arrays.copyOfRange(nums, i + 1, j + 1);
            first.putIfAbsent(prefix, j);
        }
        return null;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(subarraySum(new int[]{1, 2, 3, 4, 5}, 9)));
        // [2, 3, 4]
    }
}
