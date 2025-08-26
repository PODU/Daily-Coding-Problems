// Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
// Time O(n * wordLen), Space O(words * wordLen).
import java.util.*;

public class Solution {
    static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<>();
        if (words.length == 0 || s.isEmpty()) return res;
        int wl = words[0].length(), nw = words.length, total = wl * nw, n = s.length();
        if (total > n) return res;
        Map<String,Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);
        for (int i = 0; i < wl; ++i) {
            int left = i, count = 0;
            Map<String,Integer> window = new HashMap<>();
            for (int j = i; j + wl <= n; j += wl) {
                String w = s.substring(j, j + wl);
                if (need.containsKey(w)) {
                    window.merge(w, 1, Integer::sum); count++;
                    while (window.get(w) > need.get(w)) {
                        String lw = s.substring(left, left + wl);
                        window.merge(lw, -1, Integer::sum); count--; left += wl;
                    }
                    if (count == nw) {
                        res.add(left);
                        String lw = s.substring(left, left + wl);
                        window.merge(lw, -1, Integer::sum); count--; left += wl;
                    }
                } else {
                    window.clear(); count = 0; left = j + wl;
                }
            }
        }
        Collections.sort(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(findSubstring("dogcatcatcodecatdog", new String[]{"cat","dog"}));
        System.out.println(findSubstring("barfoobazbitbyte", new String[]{"dog","cat"}));
    }
}
