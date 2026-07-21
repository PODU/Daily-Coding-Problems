// Day 1854: Shortest substring containing all chars in a set.
// Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.
import java.util.*;

public class Solution {
    static String shortestSubstring(String s, Set<Character> need) {
        Map<Character, Integer> want = new HashMap<>();
        for (char c : need) want.merge(c, 1, Integer::sum);
        int required = want.size(), formed = 0;
        Map<Character, Integer> win = new HashMap<>();
        int bestLen = Integer.MAX_VALUE, bestL = 0, l = 0;
        for (int r = 0; r < s.length(); r++) {
            char c = s.charAt(r);
            if (want.containsKey(c)) {
                win.merge(c, 1, Integer::sum);
                if (win.get(c).intValue() == want.get(c).intValue()) formed++;
            }
            while (formed == required) {
                if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
                char lc = s.charAt(l++);
                if (want.containsKey(lc)) {
                    win.merge(lc, -1, Integer::sum);
                    if (win.get(lc) < want.get(lc)) formed--;
                }
            }
        }
        return bestLen == Integer.MAX_VALUE ? null : s.substring(bestL, bestL + bestLen);
    }

    public static void main(String[] args) {
        Set<Character> set = new HashSet<>(Arrays.asList('a', 'e', 'i'));
        String res = shortestSubstring("figehaeci", set);
        System.out.println(res == null ? "null" : res); // aeci
    }
}
