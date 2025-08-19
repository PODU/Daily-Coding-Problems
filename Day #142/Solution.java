// Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.

public class Solution {
    static boolean isBalanced(String s) {
        int lo = 0, hi = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { lo++; hi++; }
            else if (c == ')') { lo--; hi--; }
            else { lo--; hi++; } // '*' as ')', '(' or empty
            if (hi < 0) return false;
            if (lo < 0) lo = 0;
        }
        return lo == 0;
    }

    public static void main(String[] args) {
        String a = "(()*", b = "(*)", c = ")*(";
        System.out.println((isBalanced(a) ? "(()*" : "") + " and " + (isBalanced(b) ? "(*)" : "")
                + " are balanced. " + (!isBalanced(c) ? ")*(" : "") + " is not balanced.");
    }
}
