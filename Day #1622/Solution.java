// Day 1622: Longest substring with at most k distinct characters.
// Sliding window with a count map. Time O(n), Space O(k).
import java.util.*;

public class Solution {
    static String longestKDistinct(String s, int k) {
        if (k <= 0) return "";
        Map<Character, Integer> cnt = new HashMap<>();
        int left = 0, bestL = 0, bestLen = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.merge(c, 1, Integer::sum);
            while (cnt.size() > k) {
                char lc = s.charAt(left);
                if (cnt.merge(lc, -1, Integer::sum) == 0) cnt.remove(lc);
                left++;
            }
            if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestL = left; }
        }
        return s.substring(bestL, bestL + bestLen);
    }

    public static void main(String[] args) {
        System.out.println("\"" + longestKDistinct("abcba", 2) + "\"");
    }
}
