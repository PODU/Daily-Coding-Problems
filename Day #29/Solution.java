// Run-length encoding/decoding in a single pass.
// Time: O(n), Space: O(n) for output.
public class Solution {
    static String encode(String s) {
        StringBuilder res = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int j = i;
            while (j < n && s.charAt(j) == s.charAt(i)) j++;
            res.append(j - i).append(s.charAt(i));
            i = j;
        }
        return res.toString();
    }

    static String decode(String s) {
        StringBuilder res = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int count = 0;
            while (i < n && Character.isDigit(s.charAt(i))) {
                count = count * 10 + (s.charAt(i) - '0');
                i++;
            }
            char c = s.charAt(i++);
            for (int r = 0; r < count; r++) res.append(c);
        }
        return res.toString();
    }

    public static void main(String[] args) {
        String input = "AAAABBBCCDAA";
        String enc = encode(input);
        System.out.println(enc);
        System.out.println(decode(enc));
    }
}
