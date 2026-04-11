// Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
// Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
import java.util.*;

public class Solution {
    static int N, M;
    static char[][] grid;
    static int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
    static List<List<Long>> placements;
    static int best;

    static void dfsWord(String w, int idx, int r, int c, long mask, List<Long> out) {
        mask |= (1L << (r * M + c));
        if (idx == w.length() - 1) { out.add(mask); return; }
        for (int k = 0; k < 4; k++) {
            int nr = r + dr[k], nc = c + dc[k];
            if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
            if ((mask & (1L << (nr * M + nc))) != 0) continue;
            if (grid[nr][nc] != w.charAt(idx + 1)) continue;
            dfsWord(w, idx + 1, nr, nc, mask, out);
        }
    }

    static void backtrack(int i, long used, int count) {
        if (count + (placements.size() - i) <= best) return;
        if (i == placements.size()) { best = Math.max(best, count); return; }
        backtrack(i + 1, used, count);
        for (long m : placements.get(i))
            if ((m & used) == 0) backtrack(i + 1, used | m, count + 1);
    }

    static int maxWords(String[] dict, char[][] board) {
        grid = board; N = board.length; M = board[0].length;
        placements = new ArrayList<>();
        for (String w : dict) {
            List<Long> tmp = new ArrayList<>();
            for (int r = 0; r < N; r++)
                for (int c = 0; c < M; c++)
                    if (grid[r][c] == w.charAt(0)) dfsWord(w, 0, r, c, 0L, tmp);
            Set<Long> masks = new HashSet<>(tmp);
            if (!masks.isEmpty()) placements.add(new ArrayList<>(masks));
        }
        best = 0;
        backtrack(0, 0L, 0);
        return best;
    }

    public static void main(String[] args) {
        String[] dict = {"eat", "rain", "in", "rat"};
        char[][] board = {{'e','a','n'},{'t','t','i'},{'a','r','a'}};
        System.out.println(maxWords(dict, board));
    }
}
