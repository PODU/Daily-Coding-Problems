// Day 103: Shortest window containing all chars of a set via sliding window with
// a required-count and a "have all" tracker. O(n) time, O(set) space.
import java.util.*;

public class Solution {
    static String minWindow(String s, Set<Character> need) {
        if (need.isEmpty()) return "";
        Map<Character, Integer> count = new HashMap<>();
        int have = 0, left = 0, bestLen = Integer.MAX_VALUE, bestStart = 0;
        for (int right = 0; right < s.length(); right++) {
            char ch = s.charAt(right);
            if (need.contains(ch) && count.merge(ch, 1, Integer::sum) == 1) have++;
            while (have == need.size()) {
                if (right - left + 1 < bestLen) { bestLen = right - left + 1; bestStart = left; }
                char lc = s.charAt(left);
                if (need.contains(lc) && count.merge(lc, -1, Integer::sum) == 0) have--;
                left++;
            }
        }
        return bestLen == Integer.MAX_VALUE ? null : s.substring(bestStart, bestStart + bestLen);
    }

    public static void main(String[] args) {
        System.out.println(minWindow("figehaeci", new HashSet<>(Arrays.asList('a', 'e', 'i'))));
        // aeci
    }
}
