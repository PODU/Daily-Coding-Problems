// Day 373: Longest run of consecutive integers formable from the list.
// Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.
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
        int[] L = {5, 2, 99, 3, 4, 1, 100};
        System.out.println(longestConsecutive(L)); // 5
    }
}
