// Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
// Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.
import java.util.ArrayList;
import java.util.List;

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
        String[] in = {"(()*", "(*)", ")*("};
        List<String> bal = new ArrayList<>(), notbal = new ArrayList<>();
        for (String s : in) (isValid(s) ? bal : notbal).add(s);
        System.out.println(String.join(" and ", bal) + " are balanced. "
                + String.join(" and ", notbal) + " is not balanced.");
        // (()* and (*) are balanced. )*( is not balanced.
    }
}
