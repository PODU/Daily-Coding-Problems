// KMP: build LPS, scan once, record every full match start. Time O(N+k), Space O(k).
import java.util.*;

public class Solution {
    static List<Integer> findAll(String s, String pat) {
        List<Integer> res = new ArrayList<>();
        int N = s.length(), k = pat.length();
        if (k == 0 || k > N) return res;
        int[] lps = new int[k];
        for (int i = 1, len = 0; i < k; ) {
            if (pat.charAt(i) == pat.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < N; ) {
            if (s.charAt(i) == pat.charAt(j)) {
                i++; j++;
                if (j == k) { res.add(i - k); j = lps[j - 1]; }
            } else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(findAll("abracadabra", "abr")); // [0, 7]
    }
}
