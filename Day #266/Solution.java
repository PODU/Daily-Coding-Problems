// Day 266: Step words. A dict word is a step word of `word` if its length is
// one greater and its letter multiset is a superset of `word`'s (diff = 1).
// Time O(D * L) over dictionary; space O(alphabet).
import java.util.*;

public class Solution {
    static boolean isStepWord(String word, String cand) {
        if (cand.length() != word.length() + 1) return false;
        int[] cnt = new int[26];
        for (char c : cand.toLowerCase().toCharArray()) cnt[c - 'a']++;
        for (char c : word.toLowerCase().toCharArray())
            if (--cnt[c - 'a'] < 0) return false;
        int extra = 0;
        for (int v : cnt) extra += v;
        return extra == 1;
    }

    static List<String> stepWords(String word, String[] dict) {
        List<String> res = new ArrayList<>();
        for (String w : dict)
            if (isStepWord(word, w)) res.add(w);
        return res;
    }

    public static void main(String[] args) {
        String word = "APPLE";
        String[] dict = {"APPEAL", "APPLES", "KELP", "PALE", "APPLE"};
        System.out.println(stepWords(word, dict));
    }
}
