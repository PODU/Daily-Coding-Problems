// Run-length encoding/decoding. Single pass over the string.
// Time: O(n) encode/decode, Space: O(n) for output.
public class Solution {
    static String encode(String s) {
        StringBuilder out = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int j = i;
            while (j < n && s.charAt(j) == s.charAt(i)) j++;
            out.append(j - i).append(s.charAt(i));
            i = j;
        }
        return out.toString();
    }

    static String decode(String s) {
        StringBuilder out = new StringBuilder();
        int n = s.length();
        for (int i = 0; i < n;) {
            int cnt = 0;
            while (i < n && Character.isDigit(s.charAt(i))) { cnt = cnt * 10 + (s.charAt(i) - '0'); i++; }
            char c = s.charAt(i++);
            for (int k = 0; k < cnt; k++) out.append(c);
        }
        return out.toString();
    }

    public static void main(String[] args) {
        String input = "AAAABBBCCDAA";
        String enc = encode(input);
        System.out.println(enc);
        System.out.println(decode(enc));
    }
}
