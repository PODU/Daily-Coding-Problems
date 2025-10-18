// Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
// count. O(n) time, O(1) space.
public class Solution {
    static boolean isBalanced(String s) {
        int low = 0, high = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { low++; high++; }
            else if (c == ')') { low--; high--; }
            else { low--; high++; }
            if (high < 0) return false;
            if (low < 0) low = 0;
        }
        return low == 0;
    }

    public static void main(String[] args) {
        String s = "(()*";
        System.out.println(isBalanced(s) ? "balanced" : "not balanced"); // balanced
    }
}
