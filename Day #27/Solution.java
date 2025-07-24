// Balanced brackets via stack. Time O(n), Space O(n).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static boolean isBalanced(String s) {
        Deque<Character> st = new ArrayDeque<>();
        for (char c : s.toCharArray()) {
            if (c == '(' || c == '[' || c == '{') {
                st.push(c);
            } else if (c == ')' || c == ']' || c == '}') {
                if (st.isEmpty()) return false;
                char open = st.pop();
                if ((c == ')' && open != '(') ||
                    (c == ']' && open != '[') ||
                    (c == '}' && open != '{')) return false;
            }
        }
        return st.isEmpty();
    }

    public static void main(String[] args) {
        assert !isBalanced("([)]");
        assert !isBalanced("((()");
        System.out.println(isBalanced("([])[]({})") ? "true" : "false");
    }
}
