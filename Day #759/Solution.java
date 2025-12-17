// Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
// At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).
import java.util.*;

public class Solution {
    static boolean validOctet(String s) {
        if (s.isEmpty() || s.length() > 3) return false;
        if (s.length() > 1 && s.charAt(0) == '0') return false;
        return Integer.parseInt(s) <= 255;
    }

    static void backtrack(String s, int start, int part, List<String> cur,
                          List<String> res) {
        if (part == 4) {
            if (start == s.length())
                res.add(String.join(".", cur));
            return;
        }
        for (int len = 1; len <= 3 && start + len <= s.length(); len++) {
            String seg = s.substring(start, start + len);
            if (validOctet(seg)) {
                cur.add(seg);
                backtrack(s, start + len, part + 1, cur, res);
                cur.remove(cur.size() - 1);
            }
        }
    }

    static List<String> restoreIps(String s) {
        List<String> res = new ArrayList<>();
        backtrack(s, 0, 0, new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        List<String> res = restoreIps("2542540123");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);  // ['254.25.40.123', '254.254.0.123']
    }
}
