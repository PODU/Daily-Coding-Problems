// Generate all valid IPv4 addresses by backtracking: place 3 dots, each
// segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
import java.util.*;

public class Solution {
    static void backtrack(String s, int start, int part, List<String> cur, List<String> res) {
        if (part == 4) {
            if (start == s.length()) {
                res.add(String.join(".", cur));
            }
            return;
        }
        for (int len = 1; len <= 3 && start + len <= s.length(); len++) {
            String seg = s.substring(start, start + len);
            if (seg.length() > 1 && seg.charAt(0) == '0') break;
            if (Integer.parseInt(seg) > 255) break;
            cur.add(seg);
            backtrack(s, start + len, part + 1, cur, res);
            cur.remove(cur.size() - 1);
        }
    }

    static List<String> restoreIp(String s) {
        List<String> res = new ArrayList<>();
        backtrack(s, 0, 0, new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        String s = "2542540123";
        List<String> res = restoreIp(s);

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("'").append(res.get(i)).append("'");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
