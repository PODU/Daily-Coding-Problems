// Day 831: All start indices of substrings that are a concatenation of every word once.
// Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) ~ O(n) total.
import java.util.*;

public class Solution {
    static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<>();
        if (words.length == 0 || s.isEmpty()) return res;
        int L = words[0].length(), m = words.length, n = s.length();
        if ((long) L * m > n) return res;
        Map<String, Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);

        for (int offset = 0; offset < L; offset++) {
            int left = offset, count = 0;
            Map<String, Integer> have = new HashMap<>();
            for (int right = offset; right + L <= n; right += L) {
                String w = s.substring(right, right + L);
                if (need.containsKey(w)) {
                    have.merge(w, 1, Integer::sum);
                    count++;
                    while (have.get(w) > need.get(w)) {
                        String lw = s.substring(left, left + L);
                        have.merge(lw, -1, Integer::sum);
                        left += L;
                        count--;
                    }
                    if (count == m) {
                        res.add(left);
                        String lw = s.substring(left, left + L);
                        have.merge(lw, -1, Integer::sum);
                        left += L;
                        count--;
                    }
                } else {
                    have.clear();
                    count = 0;
                    left = right + L;
                }
            }
        }
        Collections.sort(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(findSubstring("dogcatcatcodecatdog", new String[]{"cat", "dog"})); // [0, 13]
        System.out.println(findSubstring("barfoobazbitbyte", new String[]{"dog", "cat"}));    // []
    }
}
