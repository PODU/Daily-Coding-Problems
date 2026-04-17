// Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
// Time O(n), Space O(1).
import java.util.*;

public class Solution {
    static boolean isValid(String s) {
        int lo = 0, hi = 0;
        for (char c : s.toCharArray()) {
            if (c == '(') { lo++; hi++; }
            else if (c == ')') { lo--; hi--; }
            else { lo--; hi++; }
            if (hi < 0) return false;
            if (lo < 0) lo = 0;
        }
        return lo == 0;
    }

    public static void main(String[] args) {
        String[] tests = {"(()*", "(*)", ")*("};
        List<String> bal = new ArrayList<>(), notBal = new ArrayList<>();
        for (String s : tests) (isValid(s) ? bal : notBal).add(s);
        System.out.println(String.join(" and ", bal) + " are balanced. "
                + String.join(" and ", notBal) + " is not balanced.");
    }
}
