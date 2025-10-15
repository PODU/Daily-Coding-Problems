// Day 437: Shortest substring containing all chars of a set via sliding window.
// O(N) time, O(set) space.
import java.util.*;

public class Solution {
    static String shortestSubstring(String s, Set<Character> chars) {
        if (chars.isEmpty()) return "";
        Map<Character,Integer> need = new HashMap<>();
        for (char c : chars) need.put(c, 0);
        int required = chars.size(), formed = 0;
        int bestLen = Integer.MAX_VALUE, bestL = 0, l = 0;
        for (int r = 0; r < s.length(); r++) {
            char c = s.charAt(r);
            if (need.containsKey(c)) {
                if (need.get(c) == 0) formed++;
                need.put(c, need.get(c) + 1);
            }
            while (formed == required) {
                if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
                char lc = s.charAt(l);
                if (need.containsKey(lc)) {
                    need.put(lc, need.get(lc) - 1);
                    if (need.get(lc) == 0) formed--;
                }
                l++;
            }
        }
        return bestLen == Integer.MAX_VALUE ? null : s.substring(bestL, bestL + bestLen);
    }

    public static void main(String[] args) {
        String s = "figehaeci";
        Set<Character> chars = new HashSet<>(Arrays.asList('a','e','i'));
        String res = shortestSubstring(s, chars);
        System.out.println(res == null ? "null" : "\"" + res + "\""); // "aeci"
    }
}
