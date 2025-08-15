// Day 123: Validate whether a string is a number (int/real/scientific).
// Single linear scan state machine. O(n) time, O(1) space.
public class Solution {
    static boolean isNumber(String s) {
        int i = 0, n = s.length();
        if (n == 0) return false;
        if (s.charAt(i) == '+' || s.charAt(i) == '-') i++;
        int digits = 0, dots = 0;
        while (i < n && (Character.isDigit(s.charAt(i)) || s.charAt(i) == '.')) {
            if (s.charAt(i) == '.') dots++;
            else digits++;
            i++;
        }
        if (dots > 1 || digits == 0) return false;
        if (i < n && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            i++;
            if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
            int expd = 0;
            while (i < n && Character.isDigit(s.charAt(i))) { expd++; i++; }
            if (expd == 0) return false;
        }
        return i == n;
    }

    public static void main(String[] args) {
        String[] tests = {"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"};
        for (String t : tests)
            System.out.println("\"" + t + "\" -> " + isNumber(t));
    }
}
