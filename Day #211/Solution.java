// Day 211: All occurrences of pattern in string via KMP.
// Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
import java.util.*;

public class Solution {
    static List<Integer> findOccurrences(String s, String p) {
        List<Integer> res = new ArrayList<>();
        int m = p.length(), n = s.length();
        if (m == 0 || m > n) return res;
        int[] lps = new int[m];
        for (int i = 1, len = 0; i < m;) {
            if (p.charAt(i) == p.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < n;) {
            if (s.charAt(i) == p.charAt(j)) {
                i++; j++;
                if (j == m) { res.add(i - m); j = lps[j - 1]; }
            } else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(findOccurrences("abracadabra", "abr"));
    }
}
