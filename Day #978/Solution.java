// Valid number check via single-pass finite-state parser.
// Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
// Time: O(n); Space: O(1).
public class Solution {
    static boolean isNumber(String s) {
        int i = 0, n = s.length();
        if (n == 0) return false;
        if (s.charAt(i) == '+' || s.charAt(i) == '-') i++;

        boolean digitsBefore = false, digitsAfter = false;
        while (i < n && Character.isDigit(s.charAt(i))) { i++; digitsBefore = true; }
        if (i < n && s.charAt(i) == '.') {
            i++;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; digitsAfter = true; }
        }
        if (!digitsBefore && !digitsAfter) return false;     // mantissa needs a digit

        if (i < n && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            i++;
            if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
            boolean expDigits = false;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; expDigits = true; }
            if (!expDigits) return false;                    // exponent needs a digit
        }
        return i == n;                                       // no trailing junk
    }

    public static void main(String[] args) {
        String[] valid = {"10", "-10", "10.1", "-10.1", "1e5"};
        String[] invalid = {"a", "x 1", "a -2", "-", "", " "};
        for (String s : valid)   System.out.println("\"" + s + "\" -> " + isNumber(s));
        for (String s : invalid) System.out.println("\"" + s + "\" -> " + isNumber(s));
    }
}
