// Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
// AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
// Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).
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
            boolean ok = true;
            int diff = 0;
            for (int i = 0; i < 26; i++) {
                if (cnt[i] < base[i]) { ok = false; break; }
                diff += cnt[i] - base[i];
            }
            if (ok && diff == 1 && !w.startsWith(word)) res.add(w);
        }
        return res;
    }

    public static void main(String[] args) {
        String word = "APPLE";
        String[] dict = {"APPEAL","APPLE","PEAR","PALE","APPEALS","PAPER","APPLES","LAPEL"};
        for (String w : stepWords(word, dict)) System.out.println(w);
    }
}
