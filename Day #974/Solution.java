// Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
// Approach: single scan with a sign/result stack. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static long evaluate(String s) {
        long result = 0, num = 0;
        int sign = 1;
        Deque<Long> st = new ArrayDeque<>();
        for (char c : s.toCharArray()) {
            if (Character.isDigit(c)) {
                num = num * 10 + (c - '0');
            } else if (c == '+') {
                result += sign * num; num = 0; sign = 1;
            } else if (c == '-') {
                result += sign * num; num = 0; sign = -1;
            } else if (c == '(') {
                st.push(result); st.push((long) sign);
                result = 0; sign = 1;
            } else if (c == ')') {
                result += sign * num; num = 0;
                long prevSign = st.pop();
                long prevResult = st.pop();
                result = prevResult + prevSign * result;
                sign = 1;
            }
        }
        result += sign * num;
        return result;
    }

    public static void main(String[] args) {
        System.out.println(evaluate("-1 + (2 + 3)")); // 4
    }
}
