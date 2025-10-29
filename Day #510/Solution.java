// Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
// Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
import java.util.*;

public class Solution {
    static boolean isPal(String s, int l, int r) {
        while (l < r) if (s.charAt(l++) != s.charAt(r--)) return false;
        return true;
    }

    static List<int[]> palindromePairs(String[] words) {
        Map<String, Integer> rev = new HashMap<>();
        for (int i = 0; i < words.length; i++) {
            rev.put(new StringBuilder(words[i]).reverse().toString(), i);
        }
        TreeSet<long[]> set = new TreeSet<>((a, b) ->
            a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            int n = w.length();
            for (int cut = 0; cut <= n; cut++) {
                if (isPal(w, 0, cut - 1)) {
                    String suf = w.substring(cut);
                    Integer j = rev.get(suf);
                    if (j != null && j != i) set.add(new long[]{j, i});
                }
                if (cut != n && isPal(w, cut, n - 1)) {
                    String pre = w.substring(0, cut);
                    Integer j = rev.get(pre);
                    if (j != null && j != i) set.add(new long[]{i, j});
                }
            }
        }
        List<int[]> res = new ArrayList<>();
        for (long[] p : set) res.add(new int[]{(int) p[0], (int) p[1]});
        return res;
    }

    public static void main(String[] args) {
        String[] words = {"code", "edoc", "da", "d"};
        List<int[]> pairs = palindromePairs(words);
        StringBuilder sb = new StringBuilder("[");
        for (int k = 0; k < pairs.size(); k++) {
            sb.append("(").append(pairs.get(k)[0]).append(", ").append(pairs.get(k)[1]).append(")");
            if (k + 1 < pairs.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
