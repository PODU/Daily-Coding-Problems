// Day 1291: Next permutation of an integer's digits.
// Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.
public class Solution {
    static String nextPermutation(String str) {
        char[] s = str.toCharArray();
        int n = s.length, i = n - 2;
        while (i >= 0 && s[i] >= s[i + 1]) i--;
        if (i < 0) return str; // already largest
        int j = n - 1;
        while (s[j] <= s[i]) j--;
        char t = s[i]; s[i] = s[j]; s[j] = t;
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            t = s[l]; s[l] = s[r]; s[r] = t;
        }
        return new String(s);
    }

    public static void main(String[] args) {
        System.out.println(nextPermutation("48975")); // 49578
    }
}
