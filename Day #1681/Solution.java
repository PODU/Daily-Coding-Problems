// Day 1681: Min parentheses to remove for validity. Track unmatched '(' and count
// unmatched ')'; answer = leftover open + unmatched close. Time O(n), Space O(1).
public class Solution {
    static int minRemovals(String s) {
        int open = 0, removals = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') open++;
            else if (c == ')') { if (open > 0) open--; else removals++; }
        }
        return removals + open;
    }

    public static void main(String[] args) {
        System.out.println(minRemovals("()())()")); // 1
        System.out.println(minRemovals(")("));       // 2
    }
}
