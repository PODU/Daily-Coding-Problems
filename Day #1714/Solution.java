// Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
import java.util.*;

public class Solution {
    static int n;
    static char[][] board;
    static boolean[][] taken;
    static int[] DR = {-1, 1, 0, 0};
    static int[] DC = {0, 0, -1, 1};
    static String[] words;
    static Set<String> used = new HashSet<>();
    static int best = 0;

    static void dfs(int r, int c, String w, int idx, Set<Integer> path, Set<Set<Integer>> found) {
        if (r < 0 || r >= n || c < 0 || c >= n) return;
        if (taken[r][c] || path.contains(r * n + c) || board[r][c] != w.charAt(idx)) return;
        path.add(r * n + c);
        if (idx == w.length() - 1) {
            found.add(new HashSet<>(path));
        } else {
            for (int d = 0; d < 4; d++)
                dfs(r + DR[d], c + DC[d], w, idx + 1, path, found);
        }
        path.remove(r * n + c);
    }

    static List<Set<Integer>> findPlacements(String w) {
        Set<Set<Integer>> found = new HashSet<>();
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                dfs(i, j, w, 0, new HashSet<>(), found);
        return new ArrayList<>(found);
    }

    static void search() {
        best = Math.max(best, used.size());
        for (String w : words) {
            if (used.contains(w)) continue;
            for (Set<Integer> placement : findPlacements(w)) {
                for (int cell : placement) taken[cell / n][cell % n] = true;
                used.add(w);
                search();
                used.remove(w);
                for (int cell : placement) taken[cell / n][cell % n] = false;
            }
        }
    }

    public static void main(String[] args) {
        board = new char[][]{{'e', 'a', 'n'}, {'t', 't', 'i'}, {'a', 'r', 'a'}};
        n = 3;
        taken = new boolean[n][n];
        words = new String[]{"eat", "rain", "in", "rat"};
        search();
        System.out.println(best);
    }
}
