// Day 857: Depth of tree from (lr) string representation.
// Approach: depth equals the maximum nesting level of parentheses.
// Time: O(n), Space: O(1).
public class Solution {
    static int depth(String s) {
        int cur = 0, mx = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { cur++; mx = Math.max(mx, cur); }
            else if (c == ')') cur--;
        }
        return mx;
    }

    public static void main(String[] args) {
        System.out.println(depth("(00)"));            // 1
        System.out.println(depth("((00)(00))"));      // 2
        System.out.println(depth("((((00)0)0)0)"));   // 4
    }
}
