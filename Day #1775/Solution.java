// Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
// digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
// O(P(10,6) * G) time, O(1) extra space.
import java.util.*;

public class Solution {
    static int[] code = new int[6];
    static boolean[] used = new boolean[10];

    static int[][] guesses; // each row: 6 digits then score

    static boolean checkAll() {
        for (int[] g : guesses) {
            int m = 0;
            for (int i = 0; i < 6; i++) if (code[i] == g[i]) m++;
            if (m != g[6]) return false;
        }
        return true;
    }

    static boolean rec(int pos) {
        if (pos == 6) return checkAll();
        for (int d = 0; d < 10; d++) {
            if (used[d]) continue;
            used[d] = true; code[pos] = d;
            if (rec(pos + 1)) { used[d] = false; return true; }
            used[d] = false;
        }
        return false;
    }

    static boolean consistent(int[][] gs) {
        guesses = gs;
        Arrays.fill(used, false);
        return rec(0);
    }

    static int[] mk(long num, int score) {
        int[] g = new int[7];
        for (int i = 5; i >= 0; i--) { g[i] = (int) (num % 10); num /= 10; }
        g[6] = score;
        return g;
    }

    public static void main(String[] args) {
        int[][] a = {mk(175286, 2), mk(293416, 3), mk(654321, 0)};
        int[][] b = {mk(123456, 4), mk(345678, 4), mk(567890, 4)};
        System.out.println(consistent(a) ? "True" : "False");
        System.out.println(consistent(b) ? "True" : "False");
    }
}
