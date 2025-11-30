// Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
// Time: O(n) over string length, Space: O(1).
public class Solution {
    static boolean isValidNumber(String s) {
        int i = 0, n = s.length();
        if (n == 0) return false;
        if (s.charAt(i) == '+' || s.charAt(i) == '-') i++;
        boolean digitsBefore = false, digitsAfter = false;
        while (i < n && Character.isDigit(s.charAt(i))) { i++; digitsBefore = true; }
        if (i < n && s.charAt(i) == '.') {
            i++;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; digitsAfter = true; }
        }
        if (!digitsBefore && !digitsAfter) return false;
        if (i < n && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            i++;
            if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
            boolean expDigits = false;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; expDigits = true; }
            if (!expDigits) return false;
        }
        return i == n;
    }

    public static void main(String[] args) {
        String[] tests = {"10","-10","10.1","-10.1","1e5","a","x 1","a -2","-"};
        for (String t : tests)
            System.out.println("\"" + t + "\" -> " + (isValidNumber(t) ? "True" : "False"));
    }
}
