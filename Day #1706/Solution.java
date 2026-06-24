// Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int longestConsecutive(int[] nums) {
        Set<Integer> s = new HashSet<>();
        for (int n : nums) s.add(n);
        int best = 0;
        for (int n : s) {
            if (!s.contains(n - 1)) {
                int cur = n, len = 1;
                while (s.contains(cur + 1)) { cur++; len++; }
                best = Math.max(best, len);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {100, 4, 200, 1, 3, 2};
        System.out.println(longestConsecutive(nums));
    }
}
