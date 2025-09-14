// Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
// Returns start index of first match, or -1 (False).
public class Solution {
    static int kmp(String text, String pat) {
        int n = text.length(), k = pat.length();
        if (k == 0) return 0;
        int[] lps = new int[k];
        for (int i = 1, len = 0; i < k;) {
            if (pat.charAt(i) == pat.charAt(len)) lps[i++] = ++len;
            else if (len != 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        for (int i = 0, j = 0; i < n;) {
            if (text.charAt(i) == pat.charAt(j)) {
                i++; j++;
                if (j == k) return i - j;
            } else if (j != 0) j = lps[j - 1];
            else i++;
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println(kmp("abxabcabcaby", "abcaby")); // 6
        System.out.println(kmp("abxabcabcaby", "zzz"));    // -1
    }
}
