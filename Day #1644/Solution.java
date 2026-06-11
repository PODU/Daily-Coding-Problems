// Next permutation of the digit array: find rightmost ascending pair, swap with
// next-greater suffix digit, reverse suffix. Time O(d), Space O(d).
public class Solution {
    static long nextPermutation(long num) {
        char[] s = Long.toString(num).toCharArray();
        int n = s.length;
        int i = n - 2;
        while (i >= 0 && s[i] >= s[i + 1]) i--;
        if (i < 0) return -1; // no next permutation
        int j = n - 1;
        while (s[j] <= s[i]) j--;
        char t = s[i]; s[i] = s[j]; s[j] = t;
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            char x = s[l]; s[l] = s[r]; s[r] = x;
        }
        return Long.parseLong(new String(s));
    }

    public static void main(String[] args) {
        System.out.println(nextPermutation(48975));
    }
}
