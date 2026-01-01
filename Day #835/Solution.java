// Day 835: Shortest substring containing all chars in a set.
// Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.
import java.util.*;

public class Solution {
    static String shortestSubstring(String s, Set<Character> charset) {
        Map<Character, Integer> need = new HashMap<>();
        for (char c : charset) need.put(c, 1);
        Map<Character, Integer> have = new HashMap<>();
        int required = need.size();
        int formed = 0, left = 0, bestL = 0, bestLen = Integer.MAX_VALUE;
        boolean found = false;
        for (int right = 0; right < s.length(); right++) {
            char ch = s.charAt(right);
            if (need.containsKey(ch)) {
                have.merge(ch, 1, Integer::sum);
                if (have.get(ch).equals(need.get(ch))) formed++;
            }
            while (formed == required) {
                if (right - left + 1 < bestLen) {
                    bestLen = right - left + 1;
                    bestL = left;
                    found = true;
                }
                char lc = s.charAt(left);
                if (need.containsKey(lc)) {
                    have.merge(lc, -1, Integer::sum);
                    if (have.get(lc) < need.get(lc)) formed--;
                }
                left++;
            }
        }
        return found ? s.substring(bestL, bestL + bestLen) : null;
    }

    public static void main(String[] args) {
        Set<Character> charset = new HashSet<>(Arrays.asList('a', 'e', 'i'));
        String res = shortestSubstring("figehaeci", charset);
        System.out.println(res != null ? res : "null");
    }
}
