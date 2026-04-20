// KMP substring search: build LPS array, then single scan.
// Time: O(N + k), Space: O(k).
public class Solution {
    static int kmpSearch(String s, String pat) {
        int N = s.length(), k = pat.length();
        if (k == 0) return 0;
        int[] lps = new int[k];
        for (int i = 1, len = 0; i < k; ) {
            if (pat.charAt(i) == pat.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < N; ) {
            if (s.charAt(i) == pat.charAt(j)) { i++; j++; if (j == k) return i - k; }
            else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return -1;
    }

    public static void main(String[] args) {
        String s = "abracadabra", pat = "cad";
        int idx = kmpSearch(s, pat);
        System.out.println(idx >= 0 ? idx : "False");
    }
}
