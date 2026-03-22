// Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.
import java.util.*;

public class Solution {
    static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<>();
        if (words.length == 0 || s.isEmpty()) return res;
        int wordLen = words[0].length();
        int numWords = words.length;
        int windowLen = wordLen * numWords;
        if (s.length() < windowLen) return res;

        Map<String, Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);

        for (int offset = 0; offset < wordLen; ++offset) {
            Map<String, Integer> window = new HashMap<>();
            int count = 0;
            int left = offset;
            for (int right = offset; right + wordLen <= s.length(); right += wordLen) {
                String word = s.substring(right, right + wordLen);
                if (need.containsKey(word)) {
                    window.merge(word, 1, Integer::sum);
                    count++;
                    while (window.get(word) > need.get(word)) {
                        String lw = s.substring(left, left + wordLen);
                        window.merge(lw, -1, Integer::sum);
                        count--;
                        left += wordLen;
                    }
                    if (count == numWords) {
                        res.add(left);
                        String lw = s.substring(left, left + wordLen);
                        window.merge(lw, -1, Integer::sum);
                        count--;
                        left += wordLen;
                    }
                } else {
                    window.clear();
                    count = 0;
                    left = right + wordLen;
                }
            }
        }
        Collections.sort(res);
        return res;
    }

    public static void main(String[] args) {
        String s = "dogcatcatcodecatdog";
        String[] words = {"cat", "dog"};
        List<Integer> res = findSubstring(s, words);
        System.out.println(res.toString());
    }
}
