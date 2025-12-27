// Day 807: Longest substring with at most k distinct characters.
// Sliding window + char count map; shrink left when distinct > k. Time O(N), Space O(k).
import java.util.*;

public class Solution {
    static int longestKDistinct(String s, int k) {
        if (k == 0) return 0;
        Map<Character, Integer> cnt = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.merge(c, 1, Integer::sum);
            while (cnt.size() > k) {
                char l = s.charAt(left);
                if (cnt.merge(l, -1, Integer::sum) == 0) cnt.remove(l);
                left++;
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(longestKDistinct("abcba", 2)); // 3
    }
}
