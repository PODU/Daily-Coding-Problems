// Day 362: Strobogrammatic numbers of N digits.
// Recursively build from outside in using rotatable digit pairs; skip leading 0.
// Time O(output size), Space O(N) recursion depth.
import java.util.*;

public class Solution {
    static char[][] PAIRS = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};

    static List<String> build(int n, int total) {
        if (n == 0) return new ArrayList<>(List.of(""));
        if (n == 1) return new ArrayList<>(List.of("0", "1", "8"));
        List<String> inner = build(n - 2, total);
        List<String> res = new ArrayList<>();
        for (String s : inner)
            for (char[] p : PAIRS) {
                if (n == total && p[0] == '0') continue;
                res.add(p[0] + s + p[1]);
            }
        return res;
    }

    static List<String> strobogrammatic(int n) { return build(n, n); }

    public static void main(String[] args) {
        int n = 2;
        System.out.println("N=" + n + ": " + String.join(" ", strobogrammatic(n)));
    }
}
