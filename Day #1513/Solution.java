// KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
// Time: O(N+k), Space: O(k).
import java.util.*;

public class Solution {
    static List<Integer> findAll(String s, String p) {
        int n = s.length(), k = p.length();
        List<Integer> res = new ArrayList<>();
        if (k == 0 || k > n) return res;
        int[] lps = new int[k];
        for (int i = 1, len = 0; i < k;) {
            if (p.charAt(i) == p.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < n;) {
            if (s.charAt(i) == p.charAt(j)) { i++; j++; if (j == k) { res.add(i - k); j = lps[j - 1]; } }
            else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return res;
    }

    public static void main(String[] args) {
        List<Integer> r = findAll("abracadabra", "abr");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) sb.append(r.get(i)).append(i + 1 < r.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb.toString());
    }
}
