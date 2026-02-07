// Reverse word order in-place: reverse whole char array, then reverse each word.
// Time: O(n), Space: O(1) extra.
public class Solution {
    static void reverseRange(char[] a, int i, int j) {
        while (i < j) {
            char t = a[i];
            a[i] = a[j];
            a[j] = t;
            i++;
            j--;
        }
    }

    static void reverseWords(char[] a) {
        reverseRange(a, 0, a.length - 1);
        int start = 0;
        for (int i = 0; i <= a.length; i++) {
            if (i == a.length || a[i] == ' ') {
                reverseRange(a, start, i - 1);
                start = i + 1;
            }
        }
    }

    public static void main(String[] args) {
        char[] a = "hello world here".toCharArray();
        reverseWords(a);
        System.out.println("\"" + new String(a) + "\"");
    }
}
