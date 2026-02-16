// Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
// Time O(1) (bounded by string length <= 12), Space O(1) extra.
import java.util.*;

public class Solution {
    static boolean valid(String seg) {
        if (seg.isEmpty() || seg.length() > 3) return false;
        if (seg.length() > 1 && seg.charAt(0) == '0') return false;
        return Integer.parseInt(seg) <= 255;
    }

    static void backtrack(String s, int start, int part, List<String> cur, List<String> res) {
        if (part == 4) {
            if (start == s.length()) res.add(String.join(".", cur));
            return;
        }
        for (int len = 1; len <= 3 && start + len <= s.length(); len++) {
            String seg = s.substring(start, start + len);
            if (valid(seg)) {
                cur.add(seg);
                backtrack(s, start + len, part + 1, cur, res);
                cur.remove(cur.size() - 1);
            }
        }
    }

    public static void main(String[] a) {
        String s = "2542540123";
        List<String> res = new ArrayList<>();
        backtrack(s, 0, 0, new ArrayList<>(), res);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
