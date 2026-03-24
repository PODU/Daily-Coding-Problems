// Day 1261: Palindrome pairs.
// Hashmap of words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
import java.util.*;

public class Solution {
    static boolean isPal(String s, int i, int j) {
        while (i < j) { if (s.charAt(i) != s.charAt(j)) return false; i++; j--; }
        return true;
    }

    static List<int[]> palindromePairs(String[] words) {
        Map<String,Integer> idx = new HashMap<>();
        for (int i = 0; i < words.length; i++) idx.put(words[i], i);
        TreeSet<int[]> set = new TreeSet<>((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        for (int i = 0; i < words.length; i++) {
            String w = words[i];
            int n = w.length();
            for (int j = 0; j <= n; j++) {
                if (isPal(w, 0, j - 1)) {
                    String back = new StringBuilder(w.substring(j)).reverse().toString();
                    Integer k = idx.get(back);
                    if (k != null && k != i) set.add(new int[]{k, i});
                }
                if (j != n && isPal(w, j, n - 1)) {
                    String back = new StringBuilder(w.substring(0, j)).reverse().toString();
                    Integer k = idx.get(back);
                    if (k != null && k != i) set.add(new int[]{i, k});
                }
            }
        }
        return new ArrayList<>(set);
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
        System.out.println(sb);
    }
}
