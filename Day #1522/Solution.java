// Concatenation of all equal-length words: sliding window per offset (0..L-1).
// Time O(|s| * L), Space O(words). Each word used exactly once.
import java.util.*;

public class Solution {
    static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<>();
        if (words.length == 0) return res;
        int L = words[0].length(), k = words.length, n = s.length();
        int total = L * k;
        if (total > n) return res;
        Map<String, Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);
        for (int off = 0; off < L; off++) {
            int left = off, count = 0;
            Map<String, Integer> win = new HashMap<>();
            for (int j = off; j + L <= n; j += L) {
                String w = s.substring(j, j + L);
                if (need.containsKey(w)) {
                    win.merge(w, 1, Integer::sum);
                    count++;
                    while (win.get(w) > need.get(w)) {
                        String lw = s.substring(left, left + L);
                        win.merge(lw, -1, Integer::sum);
                        left += L;
                        count--;
                    }
                    if (count == k) {
                        res.add(left);
                        String lw = s.substring(left, left + L);
                        win.merge(lw, -1, Integer::sum);
                        left += L;
                        count--;
                    }
                } else {
                    win.clear();
                    count = 0;
                    left = j + L;
                }
            }
        }
        Collections.sort(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(findSubstring("dogcatcatcodecatdog", new String[]{"cat", "dog"}));
        System.out.println(findSubstring("barfoobazbitbyte", new String[]{"dog", "cat"}));
    }
}
