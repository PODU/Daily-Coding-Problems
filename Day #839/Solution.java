// Day 839: Max number of dictionary words packed on an NxN board.
// For each word collect all valid adjacent-path placements (DFS), then backtrack
// over words choosing a disjoint set to maximize the count.
// Time O(exponential worst-case) on small boards; placement search bounded by board size.
import java.util.*;

public class Solution {
    static int R, C;
    static int best;
    static List<List<Long>> wordPlacements;

    static void dfs(char[][] mat, String word, int r, int c, int idx, long used,
                    Set<Long> placements) {
        if (mat[r][c] != word.charAt(idx)) return;
        used |= (1L << (r * C + c));
        if (idx == word.length() - 1) {
            placements.add(used);
            return;
        }
        int[] dr = {1, -1, 0, 0}, dc = {0, 0, 1, -1};
        for (int d = 0; d < 4; d++) {
            int nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < R && nc >= 0 && nc < C && (used & (1L << (nr * C + nc))) == 0)
                dfs(mat, word, nr, nc, idx + 1, used, placements);
        }
    }

    static List<Long> findPlacements(char[][] mat, String word) {
        Set<Long> placements = new HashSet<>();
        for (int i = 0; i < R; i++)
            for (int j = 0; j < C; j++)
                dfs(mat, word, i, j, 0, 0L, placements);
        return new ArrayList<>(placements);
    }

    static void backtrack(int i, long occupied, int count) {
        best = Math.max(best, count);
        if (i == wordPlacements.size()) return;
        backtrack(i + 1, occupied, count); // skip
        for (long tiles : wordPlacements.get(i))
            if ((occupied & tiles) == 0)
                backtrack(i + 1, occupied | tiles, count + 1);
    }

    static int maxWords(char[][] mat, String[] dict) {
        wordPlacements = new ArrayList<>();
        for (String w : dict) {
            List<Long> p = findPlacements(mat, w);
            if (!p.isEmpty()) wordPlacements.add(p);
        }
        best = 0;
        backtrack(0, 0L, 0);
        return best;
    }

    public static void main(String[] args) {
        char[][] mat = {
            {'e', 'a', 'n'},
            {'t', 't', 'i'},
            {'a', 'r', 'a'},
        };
        R = mat.length;
        C = mat[0].length;
        String[] dict = {"eat", "rain", "in", "rat"};
        System.out.println(maxWords(mat, dict));
    }
}
