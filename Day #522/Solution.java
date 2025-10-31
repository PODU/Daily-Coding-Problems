// KMP: build prefix-function for pattern, scan text. O(n+m) time, O(m) space.
import java.util.*;

public class Solution {
    static List<Integer> kmpSearch(String s, String p) {
        int m = p.length();
        int[] lps = new int[m];
        for (int i = 1, len = 0; i < m;) {
            if (p.charAt(i) == p.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        List<Integer> res = new ArrayList<>();
        for (int i = 0, j = 0; i < s.length();) {
            if (s.charAt(i) == p.charAt(j)) {
                i++; j++;
                if (j == m) { res.add(i - m); j = lps[j - 1]; }
            } else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(kmpSearch("abracadabra", "abr"));
    }
}
