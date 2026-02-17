// Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
// Time O(n*k^2), Space O(n*k).
import java.util.*;

public class Solution {
    static boolean isPal(String s, int i, int j) {
        while (i < j) { if (s.charAt(i) != s.charAt(j)) return false; i++; j--; }
        return true;
    }

    public static void main(String[] args) {
        String[] words = {"code", "edoc", "da", "d"};
        Map<String, Integer> d = new HashMap<>();
        for (int i = 0; i < words.length; i++) d.put(words[i], i);
        TreeSet<int[]> res = new TreeSet<>((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        for (int i = 0; i < words.length; i++) {
            String w = words[i]; int L = w.length();
            for (int j = 0; j <= L; j++) {
                if (isPal(w, 0, j - 1)) {
                    String r = new StringBuilder(w.substring(j)).reverse().toString();
                    Integer k = d.get(r);
                    if (k != null && k != i) res.add(new int[]{k, i});
                }
                if (j != L && isPal(w, j, L - 1)) {
                    String l = new StringBuilder(w.substring(0, j)).reverse().toString();
                    Integer k = d.get(l);
                    if (k != null && k != i) res.add(new int[]{i, k});
                }
            }
        }
        StringBuilder sb = new StringBuilder("[");
        boolean first = true;
        for (int[] p : res) { if (!first) sb.append(", "); sb.append("(" + p[0] + ", " + p[1] + ")"); first = false; }
        sb.append("]");
        System.out.println(sb);
    }
}
