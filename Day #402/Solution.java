// Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
// Time O(5^(N/2)) results, Space O(N) recursion depth.
import java.util.*;

public class Solution {
    static char[][] PAIRS = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};

    static List<String> build(int n, int total) {
        if (n == 0) return new ArrayList<>(List.of(""));
        if (n == 1) return new ArrayList<>(List.of("0", "1", "8"));
        List<String> inner = build(n - 2, total);
        List<String> res = new ArrayList<>();
        for (String s : inner) {
            for (char[] p : PAIRS) {
                if (n == total && p[0] == '0') continue; // no leading zero
                res.add(p[0] + s + p[1]);
            }
        }
        return res;
    }

    public static void main(String[] args) {
        List<String> res = build(2, 2);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("\"").append(res.get(i)).append("\"");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
