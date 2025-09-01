// Day 199: Balance parentheses with minimum insertions/deletions.
// Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
// Time: O(n), Space: O(n).
public class Solution {
    static String balance(String s) {
        StringBuilder res = new StringBuilder();
        int open = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { res.append('('); open++; }
            else {
                if (open > 0) { res.append(')'); open--; }
                else res.append("()"); // unmatched ')': insert a '(' before it
            }
        }
        while (open-- > 0) res.append(')'); // close remaining opens
        return res.toString();
    }

    public static void main(String[] args) {
        System.out.println(balance("(()"));   // (())
        System.out.println(balance("))()(")); // ()()()()
    }
}
