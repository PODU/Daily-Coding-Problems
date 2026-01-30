// Day 993: Majority element (the value occurring more than floor(n/2) times).
// Count occurrences in a hash map and return the most frequent value. This also
// yields the expected answer for the README's (non-strict) example. O(n) time/space.
import java.util.*;

public class Solution {
    static int majority(int[] nums) {
        Map<Integer, Integer> freq = new HashMap<>();
        int best = nums[0], bestCount = 0;
        for (int x : nums) {
            int c = freq.merge(x, 1, Integer::sum);
            if (c > bestCount) { bestCount = c; best = x; }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(majority(new int[]{1, 2, 1, 1, 3, 4, 0})); // 1
    }
}
