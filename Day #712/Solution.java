// Day 712: Balanced brackets check using a stack. Time O(n), space O(n).
import java.util.*;

public class Solution {
    static boolean balanced(String s) {
        Deque<Character> st = new ArrayDeque<>();
        Map<Character, Character> match = Map.of(')', '(', ']', '[', '}', '{');
        for (char c : s.toCharArray()) {
            if (c == '(' || c == '[' || c == '{') st.push(c);
            else if (match.containsKey(c)) {
                if (st.isEmpty() || st.pop() != match.get(c)) return false;
            }
        }
        return st.isEmpty();
    }

    public static void main(String[] args) {
        System.out.println(balanced("([])[]({})"));
        System.out.println(balanced("([)]"));
        System.out.println(balanced("((()"));
    }
}
