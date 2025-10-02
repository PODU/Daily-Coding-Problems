// Tree depth from nested-paren string: scan, track paren nesting, return max depth. O(N) time, O(1) space.
public class Solution {
    static int treeDepth(String s) {
        int depth = 0, maxDepth = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { depth++; maxDepth = Math.max(maxDepth, depth); }
            else if (c == ')') depth--;
        }
        return maxDepth;
    }
    public static void main(String[] args) {
        System.out.println(treeDepth("((((00)0)0)0)"));
    }
}
