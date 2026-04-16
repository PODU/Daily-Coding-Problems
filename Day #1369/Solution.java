// Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
// Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").
import java.util.*;

public class Solution {
    static boolean valid(String s) {
        if (s.isEmpty() || s.length() > 3) return false;
        if (s.length() > 1 && s.charAt(0) == '0') return false;
        return Integer.parseInt(s) <= 255;
    }

    static void bt(String s, int start, int part, List<String> cur, List<String> res) {
        if (part == 4) {
            if (start == s.length()) res.add(String.join(".", cur));
            return;
        }
        for (int len = 1; len <= 3 && start + len <= s.length(); len++) {
            String seg = s.substring(start, start + len);
            if (!valid(seg)) continue;
            cur.add(seg);
            bt(s, start + len, part + 1, cur, res);
            cur.remove(cur.size() - 1);
        }
    }

    static List<String> restore(String s) {
        List<String> res = new ArrayList<>();
        bt(s, 0, 0, new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        List<String> res = restore("2542540123");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
