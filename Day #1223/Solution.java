// Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
// O(n) time, O(1) space.
public class Solution {
    static int treeDepth(String s) {
        int depth = 0, cur = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') { cur++; depth = Math.max(depth, cur); }
            else if (c == ')') cur--;
        }
        return depth;
    }

    public static void main(String[] args) {
        System.out.println(treeDepth("((((00)0)0)0)"));
    }
}
