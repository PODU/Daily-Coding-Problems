// Greedy: track low/high possible open-paren counts in one pass.
// Time O(n), Space O(1). Balanced iff low reaches 0 at the end and high never < 0.
public class Solution {
    static boolean isBalanced(String s) {
        int low = 0, high = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') { low++; high++; }
            else if (c == ')') { low--; high--; }
            else { low--; high++; }
            if (high < 0) return false;
            if (low < 0) low = 0;
        }
        return low == 0;
    }

    public static void main(String[] args) {
        String a = "(()*", b = "(*)", c = ")*(";
        boolean ra = isBalanced(a), rb = isBalanced(b), rc = isBalanced(c);
        System.out.println(a + " and " + b + " are "
            + ((ra && rb) ? "balanced" : "not balanced") + ". "
            + c + " is " + (rc ? "balanced" : "not balanced") + ".");
    }
}
