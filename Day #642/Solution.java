// Day 642: Step words (add one letter + anagram).
// Approach: a dict word qualifies if len == len(word)+1 and its letter counts
// minus the input's are all >= 0 with exactly one extra letter.
// Time: O(D * L), Space: O(1) (26-letter counts).
import java.util.*;

public class Solution {
    static boolean isStep(String word, String cand) {
        if (cand.length() != word.length() + 1) return false;
        int[] cnt = new int[26];
        for (char c : cand.toCharArray()) cnt[c - 'A']++;
        for (char c : word.toCharArray()) if (--cnt[c - 'A'] < 0) return false;
        int extra = 0;
        for (int x : cnt) extra += x;
        return extra == 1;
    }

    static List<String> stepWords(String word, String[] dict) {
        List<String> res = new ArrayList<>();
        for (String w : dict) if (isStep(word, w)) res.add(w);
        return res;
    }

    public static void main(String[] args) {
        String word = "APPLE";
        String[] dict = {"APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"};
        System.out.println(stepWords(word, dict)); // [APPEAL, APPLES]
    }
}
