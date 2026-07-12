// Palindrome pairs: map word->index, split each word, match palindromic halves.
// Time O(N*L^2), Space O(N*L).
import java.util.*;

public class Solution {
    static boolean isPal(String s, int l, int r) {
        while (l < r) { if (s.charAt(l) != s.charAt(r)) return false; l++; r--; }
        return true;
    }

    static List<int[]> palindromePairs(String[] words) {
        Map<String,Integer> idx = new HashMap<>();
        for (int i = 0; i < words.length; i++) idx.put(words[i], i);
        List<int[]> res = new ArrayList<>();
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            int n = w.length();
            for (int j = 0; j <= n; j++) {
                if (isPal(w, 0, j - 1)) {
                    String rs = new StringBuilder(w.substring(j)).reverse().toString();
                    Integer k = idx.get(rs);
                    if (k != null && k != i) res.add(new int[]{k, i});
                }
                if (j != n && isPal(w, j, n - 1)) {
                    String rp = new StringBuilder(w.substring(0, j)).reverse().toString();
                    Integer k = idx.get(rp);
                    if (k != null && k != i) res.add(new int[]{i, k});
                }
            }
        }
        res.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        List<int[]> dedup = new ArrayList<>();
        for (int[] p : res) {
            if (dedup.isEmpty() || dedup.get(dedup.size()-1)[0] != p[0]
                    || dedup.get(dedup.size()-1)[1] != p[1]) dedup.add(p);
        }
        return dedup;
    }

    public static void main(String[] args) {
        String[] words = {"code", "edoc", "da", "d"};
        List<int[]> res = palindromePairs(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
