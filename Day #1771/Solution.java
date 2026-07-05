// Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
// Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static int evaluate(String s) {
        long result = 0;
        int sign = 1;
        Deque<long[]> st = new ArrayDeque<>(); // saved {result, sign} at '('
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (Character.isDigit(c)) {
                result += (long) sign * (c - '0');
                sign = 1;
            } else if (c == '+') {
                sign = 1;
            } else if (c == '-') {
                sign = -1;
            } else if (c == '(') {
                st.push(new long[]{result, sign});
                result = 0;
                sign = 1;
            } else if (c == ')') {
                long[] prev = st.pop();
                result = prev[0] + prev[1] * result;
                sign = 1;
            }
        }
        return (int) result;
    }

    public static void main(String[] args) {
        System.out.println(evaluate("-1 + (2 + 3)"));
    }
}
