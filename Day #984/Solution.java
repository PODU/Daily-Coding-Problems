// Balance a parentheses string with minimum insertions+deletions (insertion-only
// greedy is provably optimal: each unmatched paren needs exactly one edit).
// Time: O(n), Space: O(n).
public class Solution {
    static String balance(String s) {
        StringBuilder res = new StringBuilder();
        int bal = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') {
                res.append('(');
                bal++;
            } else { // ')'
                if (bal > 0) {
                    res.append(')');
                    bal--;
                } else {
                    res.append("()"); // insert '(' to match this ')'
                }
            }
        }
        for (int i = 0; i < bal; i++) res.append(')'); // close open '('
        return res.toString();
    }

    public static void main(String[] args) {
        System.out.println(balance("(()"));
        System.out.println(balance("))()("));
    }
}
