// Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
// pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).
import java.util.*;

public class Solution {
    static final char[][] PAIRS = {{'0','0'},{'1','1'},{'8','8'},{'6','9'},{'9','6'}};

    static List<String> build(int n, int total) {
        if (n == 0) return new ArrayList<>(List.of(""));
        if (n == 1) return new ArrayList<>(List.of("0", "1", "8"));
        List<String> inner = build(n - 2, total);
        List<String> res = new ArrayList<>();
        for (String s : inner)
            for (char[] p : PAIRS) {
                if (p[0] == '0' && n == total) continue; // no leading zero
                res.add(p[0] + s + p[1]);
            }
        return res;
    }

    static List<String> strobogrammatic(int n) { return build(n, n); }

    public static void main(String[] args) {
        System.out.println("N=2: " + strobogrammatic(2));
        System.out.println("N=1: " + strobogrammatic(1));
    }
}
