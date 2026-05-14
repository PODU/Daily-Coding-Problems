// Reverse words in a space-delimited string.
// Approach: reverse whole char array, then reverse each word in place (in-place on mutable char[]).
// Time: O(n), Space: O(1) extra.
public class Solution {
    static void reverseRange(char[] s, int i, int j) {
        while (i < j) { char t = s[i]; s[i] = s[j]; s[j] = t; i++; j--; }
    }

    static String reverseWords(String str) {
        char[] s = str.toCharArray();
        int n = s.length;
        reverseRange(s, 0, n - 1);
        int start = 0;
        for (int i = 0; i <= n; i++) {
            if (i == n || s[i] == ' ') {
                reverseRange(s, start, i - 1);
                start = i + 1;
            }
        }
        return new String(s);
    }

    public static void main(String[] args) {
        System.out.println("\"" + reverseWords("hello world here") + "\""); // "here world hello"
    }
}
