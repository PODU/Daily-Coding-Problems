// Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).
public class Solution {
    static int treeDepth(String s) {
        int depth = 0, maxDepth = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') { depth++; maxDepth = Math.max(maxDepth, depth); }
            else if (c == ')') depth--;
        }
        return maxDepth;
    }

    public static void main(String[] args) {
        String s = "((((00)0)0)0)";
        System.out.println(treeDepth(s));
    }
}
