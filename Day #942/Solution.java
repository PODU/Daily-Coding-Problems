// Day 942: Min parentheses to remove to make the string valid.
// One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.
public class Solution {
    static int minRemovals(String s) {
        int open = 0, removals = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') open++;
            else if (c == ')') {
                if (open > 0) open--;
                else removals++;
            }
        }
        return removals + open;
    }

    public static void main(String[] args) {
        System.out.println(minRemovals("()())()")); // 1
        System.out.println(minRemovals(")("));        // 2
    }
}
