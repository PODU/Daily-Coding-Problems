// Day 638: Reverse the words in a string (in-place on a char[]).
// Approach: reverse whole array, then reverse each word back.
// Time: O(n), Space: O(1) extra.
public class Solution {
    static void rev(char[] a, int i, int j) {
        while (i < j) { char t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
    }

    static String reverseWords(String s) {
        char[] a = s.toCharArray();
        int n = a.length;
        rev(a, 0, n - 1);
        int i = 0;
        while (i < n) {
            int j = i;
            while (j < n && a[j] != ' ') j++;
            rev(a, i, j - 1);
            i = j + 1;
        }
        return new String(a);
    }

    public static void main(String[] args) {
        System.out.println(reverseWords("hello world here")); // here world hello
    }
}
