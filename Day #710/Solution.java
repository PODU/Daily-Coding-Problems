// Day 710: Find start indices where s contains a concatenation of all equal-length
// words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
import java.util.*;

public class Solution {
    static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<>();
        if (words.length == 0) return res;
        int wl = words[0].length(), k = words.length, total = wl * k, n = s.length();
        if (total > n) return res;
        Map<String, Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);
        for (int off = 0; off < wl; off++) {
            int left = off, count = 0;
            Map<String, Integer> window = new HashMap<>();
            for (int j = off; j + wl <= n; j += wl) {
                String w = s.substring(j, j + wl);
                if (need.containsKey(w)) {
                    window.merge(w, 1, Integer::sum); count++;
                    while (window.get(w) > need.get(w)) {
                        String lw = s.substring(left, left + wl);
                        window.merge(lw, -1, Integer::sum); left += wl; count--;
                    }
                    if (count == k) {
                        res.add(left);
                        String lw = s.substring(left, left + wl);
                        window.merge(lw, -1, Integer::sum); left += wl; count--;
                    }
                } else {
                    window.clear(); count = 0; left = j + wl;
                }
            }
        }
        Collections.sort(res);
        return res;
    }

    static void report(String s, String[] w) {
        System.out.println(findSubstring(s, w));
    }

    public static void main(String[] args) {
        report("dogcatcatcodecatdog", new String[]{"cat", "dog"});
        report("barfoobazbitbyte", new String[]{"dog", "cat"});
    }
}
