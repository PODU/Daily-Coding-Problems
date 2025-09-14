// Day 274: Evaluate string of (), single digits, +/- without eval.
// Stack-based sign tracking. Time O(N), Space O(N).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static int evaluate(String s) {
        int result = 0, sign = 1;
        Deque<Integer> st = new ArrayDeque<>();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (Character.isDigit(c)) {
                result += sign * (c - '0');
            } else if (c == '+') {
                sign = 1;
            } else if (c == '-') {
                sign = -1;
            } else if (c == '(') {
                st.push(result);
                st.push(sign);
                result = 0;
                sign = 1;
            } else if (c == ')') {
                int s2 = st.pop();
                int prev = st.pop();
                result = prev + s2 * result;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(evaluate("-1 + (2 + 3)")); // 4
    }
}
