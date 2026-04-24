// Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
// Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int evaluate(String s) {
        int result = 0, sign = 1, i = 0, n = s.length();
        Deque<Integer> st = new ArrayDeque<>();
        st.push(1);
        while (i < n) {
            char c = s.charAt(i);
            if (Character.isDigit(c)) {
                int num = 0;
                while (i < n && Character.isDigit(s.charAt(i))) {
                    num = num * 10 + (s.charAt(i) - '0'); i++;
                }
                result += sign * st.peek() * num;
                continue;
            } else if (c == '+') sign = 1;
            else if (c == '-') sign = -1;
            else if (c == '(') { st.push(sign * st.peek()); sign = 1; }
            else if (c == ')') st.pop();
            i++;
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(evaluate("-1 + (2 + 3)")); // 4
    }
}
