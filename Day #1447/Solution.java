// Day 1447: Does a secret code (6 distinct digits) exist consistent with all
// (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
import java.util.*;

public class Solution {
    static int scoreMatch(int[] code, String guess) {
        int s = 0;
        for (int i = 0; i < 6; i++) if (code[i] == guess.charAt(i) - '0') s++;
        return s;
    }

    static boolean dfs(int pos, int[] code, boolean[] used, List<String[]> guesses) {
        if (pos == 6) {
            for (String[] g : guesses)
                if (scoreMatch(code, g[0]) != Integer.parseInt(g[1])) return false;
            return true;
        }
        for (int d = 0; d < 10; d++) {
            if (used[d]) continue;
            if (pos == 0 && d == 0) continue; // no leading zero
            used[d] = true; code[pos] = d;
            if (dfs(pos + 1, code, used, guesses)) { used[d] = false; return true; }
            used[d] = false;
        }
        return false;
    }

    static boolean consistent(List<String[]> guesses) {
        return dfs(0, new int[6], new boolean[10], guesses);
    }

    public static void main(String[] args) {
        List<String[]> e1 = Arrays.asList(
            new String[]{"175286","2"}, new String[]{"293416","3"}, new String[]{"654321","0"});
        List<String[]> e2 = Arrays.asList(
            new String[]{"123456","4"}, new String[]{"345678","4"}, new String[]{"567890","4"});
        System.out.println(consistent(e1) ? "True" : "False"); // True
        System.out.println(consistent(e2) ? "True" : "False"); // False
    }
}
