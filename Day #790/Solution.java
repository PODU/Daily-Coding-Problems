// Step words: a dict word qualifies if len==word.len+1 and word's letter
// multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
import java.util.*;

public class Solution {
    static List<String> stepWords(String word, String[] dict) {
        int[] base = new int[26];
        for (char c : word.toCharArray()) base[c - 'A']++;
        List<String> res = new ArrayList<>();
        for (String w : dict) {
            if (w.length() != word.length() + 1) continue;
            int[] cnt = new int[26];
            for (char c : w.toCharArray()) cnt[c - 'A']++;
            int extra = 0; boolean ok = true;
            for (int i = 0; i < 26; i++) {
                int d = cnt[i] - base[i];
                if (d < 0) { ok = false; break; }
                extra += d;
            }
            if (ok && extra == 1) res.add(w);
        }
        return res;
    }

    public static void main(String[] args) {
        String word = "APPLE";
        String[] dict = {"APPEAL", "BANANA", "ORANGE", "GRAPE"};
        System.out.println(String.join(" ", stepWords(word, dict)));
    }
}
