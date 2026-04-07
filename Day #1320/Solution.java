// Longest substring with at most k distinct characters via a sliding window
// with a char-count map. Time O(n), Space O(k).
import java.util.*;

public class Solution {
    static String longestKDistinct(String s, int k) {
        Map<Character,Integer> cnt = new HashMap<>();
        int left = 0, bestStart = 0, bestLen = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.merge(c, 1, Integer::sum);
            while (cnt.size() > k) {
                char l = s.charAt(left);
                if (cnt.merge(l, -1, Integer::sum) == 0) cnt.remove(l);
                left++;
            }
            if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestStart = left; }
        }
        return s.substring(bestStart, bestStart + bestLen);
    }

    public static void main(String[] args) {
        String sub = longestKDistinct("abcba", 2);
        System.out.println("The longest substring with k distinct characters is \"" + sub + "\".");
    }
}
