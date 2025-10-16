// Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
// Returns the start index of the first match, or -1 (False) if absent.
public class Solution {
    static int[] buildLPS(String p) {
        int k = p.length();
        int[] lps = new int[k];
        int len = 0;
        for (int i = 1; i < k; ) {
            if (p.charAt(i) == p.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        return lps;
    }

    static int kmpSearch(String text, String pat) {
        if (pat.isEmpty()) return 0;
        int[] lps = buildLPS(pat);
        int i = 0, j = 0, n = text.length(), k = pat.length();
        while (i < n) {
            if (text.charAt(i) == pat.charAt(j)) { i++; j++; if (j == k) return i - k; }
            else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return -1;
    }

    public static void main(String[] args) {
        String text = "abxabcabcaby", pat = "abcaby";
        int idx = kmpSearch(text, pat);
        System.out.println(idx == -1 ? "False" : idx); // 6
    }
}
