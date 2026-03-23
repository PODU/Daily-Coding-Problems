// Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static boolean isBalanced(String s) {
        Deque<Character> st = new ArrayDeque<>();
        Map<Character,Character> pair = Map.of(')','(', ']','[', '}','{');
        for (char c : s.toCharArray()) {
            if (c == '(' || c == '[' || c == '{') st.push(c);
            else if (pair.containsKey(c)) {
                if (st.isEmpty() || st.pop() != pair.get(c)) return false;
            }
        }
        return st.isEmpty();
    }

    public static void main(String[] args) {
        System.out.println(isBalanced("([])[]({})"));
        System.out.println(isBalanced("([)]"));
    }
}
