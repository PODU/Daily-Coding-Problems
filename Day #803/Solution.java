// Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
// Brute force all 6-digit distinct-digit codes, verify every guess's score.
// Time O(10^6 * G), Space O(G).
import java.util.*;

public class Solution {
    static int[] digits(int x) {
        int[] d = new int[6];
        for (int i = 5; i >= 0; i--) { d[i] = x % 10; x /= 10; }
        return d;
    }

    static boolean distinct(int[] d) {
        int seen = 0;
        for (int v : d) { if ((seen & (1 << v)) != 0) return false; seen |= 1 << v; }
        return true;
    }

    static int score(int[] code, int[] guess) {
        int s = 0;
        for (int i = 0; i < 6; i++) if (code[i] == guess[i]) s++;
        return s;
    }

    static boolean feasible(int[][] guesses) {
        int[][] gd = new int[guesses.length][];
        int[] sc = new int[guesses.length];
        for (int i = 0; i < guesses.length; i++) {
            gd[i] = digits(guesses[i][0]);
            sc[i] = guesses[i][1];
        }
        for (int code = 0; code <= 999999; code++) {
            int[] d = digits(code);
            if (!distinct(d)) continue;
            boolean ok = true;
            for (int i = 0; i < gd.length; i++)
                if (score(d, gd[i]) != sc[i]) { ok = false; break; }
            if (ok) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        System.out.println(feasible(new int[][]{{175286, 2}, {293416, 3}, {654321, 0}})); // true
        System.out.println(feasible(new int[][]{{123456, 4}, {345678, 4}, {567890, 4}})); // false
    }
}
