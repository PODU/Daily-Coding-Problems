// Approach: deterministic single linear scan validating sign/digits/dot/exponent.
// Time O(n), Space O(1).
public class Solution {
    static boolean isNumber(String s) {
        int n = s.length(), i = 0;
        if (n == 0) return false;
        char c0 = s.charAt(0);
        if (c0 == '+' || c0 == '-') i++;
        boolean digits = false, dot = false;
        while (i < n && (Character.isDigit(s.charAt(i)) || s.charAt(i) == '.')) {
            if (s.charAt(i) == '.') {
                if (dot) return false;
                dot = true;
            } else digits = true;
            i++;
        }
        if (!digits) return false;
        if (i < n && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            i++;
            if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) i++;
            boolean expDigits = false;
            while (i < n && Character.isDigit(s.charAt(i))) { expDigits = true; i++; }
            if (!expDigits) return false;
        }
        return i == n;
    }

    public static void main(String[] args) {
        String[] tests = {"10","-10","10.1","-10.1","1e5","a","x 1","a -2","-"};
        for (String t : tests) System.out.println(isNumber(t) ? "true" : "false");
    }
}
