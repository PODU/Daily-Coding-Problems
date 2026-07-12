// Balance parentheses with min insertions/deletions via insertion-based scan.
// Time O(n), Space O(n).
public class Solution {
    static String balance(String s) {
        StringBuilder result = new StringBuilder();
        int open = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { result.append('('); open++; }
            else {
                if (open == 0) { result.append('('); result.append(')'); }
                else { result.append(')'); open--; }
            }
        }
        for (int i = 0; i < open; i++) result.append(')');
        return result.toString();
    }

    public static void main(String[] args) {
        System.out.println(balance("(()"));
        System.out.println(balance("))()("));
    }
}
