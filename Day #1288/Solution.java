// Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
// Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
public class Solution {
    static String balance(String s) {
        StringBuilder res = new StringBuilder();
        int open = 0;
        for (char ch : s.toCharArray()) {
            if (ch == '(') { res.append('('); open++; }
            else {
                if (open > 0) { res.append(')'); open--; }
                else { res.append("()"); } // insert matching '(' before unmatched ')'
            }
        }
        while (open-- > 0) res.append(')');
        return res.toString();
    }

    public static void main(String[] args) {
        System.out.println(balance("(()"));    // (())
        System.out.println(balance("))()("));  // ()()()()
    }
}
