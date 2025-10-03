// Day 361: Mastermind feasibility.
// Brute-force all 6-position codes with distinct digits; accept if some code
// matches every guess's score. Time O(P*G*6), P=151200, Space O(1).
import java.util.*;

public class Solution {
    static int scoreOf(char[] a, String b) {
        int s = 0;
        for (int i = 0; i < 6; i++) if (a[i] == b.charAt(i)) s++;
        return s;
    }

    static boolean rec(char[] code, int pos, int used, List<String> g, List<Integer> sc) {
        if (pos == 6) {
            for (int i = 0; i < g.size(); i++)
                if (scoreOf(code, g.get(i)) != sc.get(i)) return false;
            return true;
        }
        for (int d = 0; d < 10; d++)
            if ((used & (1 << d)) == 0) {
                code[pos] = (char) ('0' + d);
                if (rec(code, pos + 1, used | (1 << d), g, sc)) return true;
            }
        return false;
    }

    static boolean feasible(List<String> g, List<Integer> sc) {
        return rec(new char[6], 0, 0, g, sc);
    }

    static String pad6(long n) {
        return String.format("%06d", n);
    }

    public static void main(String[] args) {
        List<String> g1 = Arrays.asList(pad6(175286), pad6(293416), pad6(654321));
        List<Integer> s1 = Arrays.asList(2, 3, 0);
        List<String> g2 = Arrays.asList(pad6(123456), pad6(345678), pad6(567890));
        List<Integer> s2 = Arrays.asList(4, 4, 4);
        System.out.println(feasible(g1, s1) ? "True" : "False");
        System.out.println(feasible(g2, s2) ? "True" : "False");
    }
}
