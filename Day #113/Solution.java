// Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
public class Solution {
    static void reverse(char[] a, int i, int j){
        while (i < j){ char t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
    }
    static String reverseWords(String str){
        char[] s = str.toCharArray();
        int n = s.length;
        reverse(s, 0, n - 1);
        int start = 0;
        for (int i = 0; i <= n; i++){
            if (i == n || s[i] == ' '){
                reverse(s, start, i - 1);
                start = i + 1;
            }
        }
        return new String(s);
    }
    public static void main(String[] args){
        System.out.println("\"" + reverseWords("hello world here") + "\"");
    }
}
