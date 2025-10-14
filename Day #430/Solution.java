// Day 430: Balance parentheses with the minimum number of insertions + deletions.
// One pass: keep matched pairs; unmatched ')' -> "()", leftover '(' gets a ')'. Time O(n), Space O(n).
public class Solution {
    static String balance(String s) {
        StringBuilder res = new StringBuilder();
        int open = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') {
                open++;
                res.append('(');
            } else { // ')'
                if (open > 0) {
                    open--;
                    res.append(')');
                } else {
                    res.append("()");
                }
            }
        }
        while (open-- > 0) res.append(')');
        return res.toString();
    }

    public static void main(String[] args) {
        System.out.println(balance("(()"));
        System.out.println(balance("))()("));
    }
}
