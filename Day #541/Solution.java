// Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
// Time O(n) encode, O(output) decode. Space O(n).
public class Solution {
    static String encode(String s) {
        StringBuilder out = new StringBuilder();
        int i = 0, n = s.length();
        while (i < n) {
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
        for (char c : s.toCharArray()) {
            if (Character.isDigit(c)) count = count * 10 + (c - '0');
            else { for (int k = 0; k < count; k++) out.append(c); count = 0; }
        }
        return out.toString();
    }

    public static void main(String[] args) {
        String original = "AAAABBBCCDAA";
        String enc = encode(original);
        System.out.println(enc);                       // 4A3B2C1D2A
        System.out.println(decode(enc).equals(original));
    }
}
