// Day 1027: Longest consecutive elements sequence.
// Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static int longestConsecutive(int[] nums) {
        Set<Integer> s = new HashSet<>();
        for (int x : nums) s.add(x);
        int best = 0;
        for (int x : s) {
            if (!s.contains(x - 1)) {
                int len = 1, cur = x;
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
