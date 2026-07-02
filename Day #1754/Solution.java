// Day 1754: All strobogrammatic numbers with N digits.
// Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
// Time O(output size), space O(N) recursion depth.
import java.util.*;

public class Solution {
    static final char[][] PAIRS = {
        {'0','0'}, {'1','1'}, {'6','9'}, {'8','8'}, {'9','6'}};

    static List<String> build(int n, int total) {
        if (n == 0) return new ArrayList<>(List.of(""));
        if (n == 1) return new ArrayList<>(List.of("0", "1", "8"));
        List<String> inner = build(n - 2, total);
        List<String> res = new ArrayList<>();
        for (String s : inner) {
            for (char[] p : PAIRS) {
                if (p[0] == '0' && n == total) continue; // no leading zero
                res.add(p[0] + s + p[1]);
            }
        }
        return res;
    }

    static List<String> strobogrammatic(int n) { return build(n, n); }

    public static void main(String[] args) {
        for (int n : new int[]{2, 3}) {
            System.out.println("N=" + n + ": " + strobogrammatic(n));
        }
    }
}
