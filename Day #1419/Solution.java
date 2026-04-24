// Day 1419: reverse the order of space-delimited words, in-place.
// Approach: reverse whole char array, then reverse each word. O(n) time, O(1) extra space.
public class Solution {
    static void reverseRange(char[] s, int i, int j) {
        while (i < j) {
            char t = s[i]; s[i] = s[j]; s[j] = t;
            i++; j--;
        }
    }

    static void reverseWords(char[] s) {
        int n = s.length;
        reverseRange(s, 0, n - 1);
        int start = 0;
        for (int i = 0; i <= n; i++) {
            if (i == n || s[i] == ' ') {
                reverseRange(s, start, i - 1);
                start = i + 1;
            }
        }
    }

    public static void main(String[] args) {
        char[] s = "hello world here".toCharArray();
        reverseWords(s);
        System.out.println(new String(s)); // here world hello
    }
}
