// Minimum parentheses to remove to make string valid: single pass counting unmatched
// open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).
public class Solution {
    static int minRemove(String s) {
        int open = 0, removals = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') open++;
            else if (c == ')') {
                if (open > 0) open--;
                else removals++;
            }
        }
        return removals + open;
    }

    public static void main(String[] args) {
        System.out.println(minRemove("()())()")); // 1
        System.out.println(minRemove(")("));        // 2
    }
}
