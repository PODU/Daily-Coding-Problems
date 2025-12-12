// Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
// Single linear scan; parentheses handled by pushing the running result and sign.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static int evaluate(String s){
        long result = 0;
        int sign = 1;
        Deque<Integer> st = new ArrayDeque<>();
        int i = 0, n = s.length();
        while(i < n){
            char c = s.charAt(i);
            if(Character.isDigit(c)){
                long num = 0;
                while(i < n && Character.isDigit(s.charAt(i))){ num = num*10 + (s.charAt(i)-'0'); i++; }
                result += sign * num;
                continue;
            } else if(c == '+'){ sign = 1; }
            else if(c == '-'){ sign = -1; }
            else if(c == '('){ st.push((int)result); st.push(sign); result = 0; sign = 1; }
            else if(c == ')'){ int s2 = st.pop(); int r2 = st.pop(); result = r2 + s2*result; }
            i++;
        }
        return (int)result;
    }

    public static void main(String[] args){
        System.out.println(evaluate("-1 + (2 + 3)")); // 4
    }
}
