// KMP string matching: build LPS failure array O(k), scan text O(N). Time O(N+k), Space O(k).
public class Solution {
    static int kmpSearch(String text, String pat) {
        int n = text.length(), k = pat.length();
        if (k == 0) return 0;
        int[] lps = new int[k];
        for (int i = 1, len = 0; i < k; ) {
            if (pat.charAt(i) == pat.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < n; ) {
            if (text.charAt(i) == pat.charAt(j)) { i++; j++; if (j == k) return i - j; }
            else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return -1;
    }

    public static void main(String[] args) {
        String text = "abxabcabcaby";
        int r1 = kmpSearch(text, "abcaby");
        System.out.println(r1 == -1 ? "Not found" : "Found at index " + r1);
        int r2 = kmpSearch(text, "xyz");
        System.out.println(r2 == -1 ? "Not found" : "Found at index " + r2);
    }
}
