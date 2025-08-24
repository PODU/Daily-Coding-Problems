// Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
import java.util.*;

public class Solution {
    static boolean isPalin(String s, int i, int j) {
        while (i < j) { if (s.charAt(i++) != s.charAt(j--)) return false; }
        return true;
    }

    public static void main(String[] args) {
        String[] words = {"code", "edoc", "da", "d"};
        Map<String, Integer> rev = new HashMap<>();
        for (int i = 0; i < words.length; i++)
            rev.put(new StringBuilder(words[i]).reverse().toString(), i);

        Set<long[]> seen = new HashSet<>();
        List<int[]> res = new ArrayList<>();
        Set<Long> dedup = new HashSet<>();
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            int n = w.length();
            for (int cut = 0; cut <= n; cut++) {
                if (isPalin(w, 0, cut - 1)) {
                    String suf = w.substring(cut);
                    Integer j = rev.get(suf);
                    if (j != null && j != i) addPair(res, dedup, j, i, words.length);
                }
                if (cut < n && isPalin(w, cut, n - 1)) {
                    String pre = w.substring(0, cut);
                    Integer j = rev.get(pre);
                    if (j != null && j != i) addPair(res, dedup, i, j, words.length);
                }
            }
        }

        res.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }

    static void addPair(List<int[]> res, Set<Long> dedup, int a, int b, int n) {
        long key = (long) a * n + b;
        if (dedup.add(key)) res.add(new int[]{a, b});
    }
}
