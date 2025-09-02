// Day 205: Next permutation of an integer's digits.
// Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
// Time: O(d), Space: O(d) for digits.
public class Solution {
    static long nextPermutation(long n) {
        char[] s = Long.toString(n).toCharArray();
        int i = s.length - 2;
        while (i >= 0 && s[i] >= s[i + 1]) i--;
        if (i < 0) return n; // already the largest permutation
        int j = s.length - 1;
        while (s[j] <= s[i]) j--;
        char t = s[i]; s[i] = s[j]; s[j] = t;
        for (int l = i + 1, r = s.length - 1; l < r; l++, r--) { t = s[l]; s[l] = s[r]; s[r] = t; }
        return Long.parseLong(new String(s));
    }

    public static void main(String[] args) {
        System.out.println(nextPermutation(48975)); // 49578
    }
}
