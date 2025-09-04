// Day 213: Generate all valid IP addresses from a digit string.
// Approach: backtracking over the 3 dot positions; each segment 1-3 digits, 0-255, no leading zeros.
// Time O(1) effectively (length <= 12), Space O(1).
import java.util.*;

public class Solution {
    static boolean valid(String seg) {
        if (seg.isEmpty() || seg.length() > 3) return false;
        if (seg.length() > 1 && seg.charAt(0) == '0') return false;
        return Integer.parseInt(seg) <= 255;
    }

    static void solve(String s, int start, int part, List<String> cur, List<String> res) {
        if (part == 4) {
            if (start == s.length()) res.add(String.join(".", cur));
            return;
        }
        for (int len = 1; len <= 3 && start + len <= s.length(); len++) {
            String seg = s.substring(start, start + len);
            if (!valid(seg)) continue;
            cur.add(seg);
            solve(s, start + len, part + 1, cur, res);
            cur.remove(cur.size() - 1);
        }
    }

    static List<String> restore(String s) {
        List<String> res = new ArrayList<>();
        solve(s, 0, 0, new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        List<String> r = restore("2542540123");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) {
            sb.append("'").append(r.get(i)).append("'");
            if (i + 1 < r.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
