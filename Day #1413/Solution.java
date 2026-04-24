// Day 1413: shortest substring of s containing all characters in a set.
// Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.
import java.util.*;

public class Solution {
    static String shortestSubstring(String s, Set<Character> want) {
        Map<Character,Integer> need = new HashMap<>();
        for (char c : want) need.merge(c, 1, Integer::sum);
        int required = need.size(), formed = 0;
        Map<Character,Integer> win = new HashMap<>();
        int bestLen = Integer.MAX_VALUE, bestL = 0, l = 0;
        for (int r = 0; r < s.length(); r++) {
            char c = s.charAt(r);
            if (need.containsKey(c)) {
                win.merge(c, 1, Integer::sum);
                if (win.get(c).equals(need.get(c))) formed++;
            }
            while (formed == required) {
                if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
                char lc = s.charAt(l);
                if (need.containsKey(lc)) {
                    win.merge(lc, -1, Integer::sum);
                    if (win.get(lc) < need.get(lc)) formed--;
                }
                l++;
            }
        }
        return bestLen == Integer.MAX_VALUE ? "null" : s.substring(bestL, bestL + bestLen);
    }

    public static void main(String[] args) {
        Set<Character> want = new HashSet<>(Arrays.asList('a','e','i'));
        System.out.println(shortestSubstring("figehaeci", want)); // aeci
    }
}
