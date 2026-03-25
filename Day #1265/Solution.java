// Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
// DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
import java.util.*;

public class Solution {
    static List<String> wordBreak(String s, Set<String> dict) {
        int n = s.length();
        int[] back = new int[n + 1];
        Arrays.fill(back, -2);
        back[0] = -1;
        for (int i = 1; i <= n; i++) {
            for (int j = 0; j < i; j++) {
                if (back[j] != -2 && dict.contains(s.substring(j, i))) { back[i] = j; break; }
            }
        }
        if (back[n] == -2) return null;
        LinkedList<String> res = new LinkedList<>();
        for (int i = n; i > 0; i = back[i]) res.addFirst(s.substring(back[i], i));
        return res;
    }

    public static void main(String[] args) {
        Set<String> dict = new HashSet<>(Arrays.asList("quick", "brown", "the", "fox"));
        System.out.println(wordBreak("thequickbrownfox", dict));
    }
}
