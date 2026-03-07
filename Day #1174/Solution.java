// Day 1174: Decide whether a string is a valid number.
// Single linear scan: optional sign, integer/fraction digits, optional exponent.
// Time O(N), Space O(1).
public class Solution {
    static boolean isNumber(String s) {
        int n = s.length(), i = 0;
        if (n == 0) return false;
        if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
        boolean before = false, after = false;
        while (i < n && Character.isDigit(s.charAt(i))) { i++; before = true; }
        if (i < n && s.charAt(i) == '.') {
            i++;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; after = true; }
        }
        if (!before && !after) return false;
        if (i < n && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            i++;
            if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
            boolean exp = false;
            while (i < n && Character.isDigit(s.charAt(i))) { i++; exp = true; }
            if (!exp) return false;
        }
        return i == n;
    }

    public static void main(String[] args) {
        String[] tests = {"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"};
        for (String t : tests)
            System.out.println("\"" + t + "\" -> " + (isNumber(t) ? "true" : "false"));
    }
}
