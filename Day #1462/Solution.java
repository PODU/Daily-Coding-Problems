// Evaluate Reverse Polish Notation using a stack.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    public static long evalRPN(String[] tokens) {
        Deque<Long> st = new ArrayDeque<>();
        for (String t : tokens) {
            switch (t) {
                case "+": case "-": case "*": case "/": {
                    long b = st.pop();
                    long a = st.pop();
                    long r;
                    if (t.equals("+")) r = a + b;
                    else if (t.equals("-")) r = a - b;
                    else if (t.equals("*")) r = a * b;
                    else r = a / b; // integer division truncates toward zero
                    st.push(r);
                    break;
                }
                default:
                    st.push(Long.parseLong(t));
            }
        }
        return st.peek();
    }

    public static void main(String[] args) {
        String[] tokens = {"15","7","1","1","+","-","/","3","*","2","1","1","+","+","-"};
        System.out.println(evalRPN(tokens));
    }
}
