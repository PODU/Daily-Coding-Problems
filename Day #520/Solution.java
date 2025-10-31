// Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
public class Solution {
    static int treeDepth(String s) {
        int depth = 0, best = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { depth++; best = Math.max(best, depth); }
            else if (c == ')') depth--;
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(treeDepth("((((00)0)0)0)")); // 4
    }
}
