// Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
// Time O(n), Space O(1).
public class Solution {
    static int minRemoval(String s) {
        int open = 0, removals = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') open++;
            else if (c == ')') {
                if (open > 0) open--;
                else removals++;       // unmatched ')'
            }
        }
        return removals + open;        // leftover unmatched '('
    }

    public static void main(String[] args) {
        System.out.println(minRemoval("()())()")); // 1
        System.out.println(minRemoval(")("));        // 2
    }
}
