// Day 1294: Run-length encoding and decoding for alphabetic strings.
// Single linear scan for each direction. O(n) time, O(n) space.
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
        int count = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (Character.isDigit(c)) count = count * 10 + (c - '0');
            else { for (int k = 0; k < count; k++) out.append(c); count = 0; }
        }
        return out.toString();
    }

    public static void main(String[] args) {
        String in = "AAAABBBCCDAA";
        String e = encode(in);
        System.out.println(e);          // 4A3B2C1D2A
        System.out.println(decode(e));  // AAAABBBCCDAA
    }
}
