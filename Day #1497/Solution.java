// Day 1497: Step words. Dict word w is a step word of s if len(w)==len(s)+1
// and multiset(s) subset of multiset(w). Char-count comparison.
// Time O(D*L), Space O(1) (26-letter alphabet).
import java.util.*;

public class Solution {
    static List<String> stepWords(String word, List<String> dict) {
        int[] base = new int[26];
        for (char c : word.toCharArray()) base[c - 'A']++;
        List<String> res = new ArrayList<>();
        for (String w : dict) {
            if (w.length() != word.length() + 1) continue;
            int[] cnt = new int[26];
            for (char c : w.toCharArray()) cnt[c - 'A']++;
            int extra = 0; boolean ok = true;
            for (int i = 0; i < 26; i++) {
                int diff = cnt[i] - base[i];
                if (diff < 0) { ok = false; break; }
                extra += diff;
            }
            if (ok && extra == 1) res.add(w);
        }
        return res;
    }

    public static void main(String[] args) {
        String input = "APPLE";
        List<String> dict = Arrays.asList("APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS");
        System.out.println(stepWords(input, dict));
    }
}
