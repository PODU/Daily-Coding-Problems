// Day 594: Max number of non-overlapping dictionary words packable on a letter board.
// Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
// then backtracking max set-packing to pick the most pairwise-disjoint placements.
// Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.
import java.util.*;

public class Solution {
    static char[][] board;
    static int R, C;
    static int best;

    static void findWord(String w, int idx, int r, int c, long mask, List<Long> out) {
        if (r < 0 || c < 0 || r >= R || c >= C) return;
        int bit = r * C + c;
        if ((mask & (1L << bit)) != 0) return;
        if (board[r][c] != w.charAt(idx)) return;
        mask |= (1L << bit);
        if (idx == w.length() - 1) { out.add(mask); return; }
        findWord(w, idx + 1, r + 1, c, mask, out);
        findWord(w, idx + 1, r - 1, c, mask, out);
        findWord(w, idx + 1, r, c + 1, mask, out);
        findWord(w, idx + 1, r, c - 1, mask, out);
    }

    static void pack(List<Long> placements, int i, long used, int cnt) {
        best = Math.max(best, cnt);
        for (int j = i; j < placements.size(); j++) {
            if ((placements.get(j) & used) == 0)
                pack(placements, j + 1, used | placements.get(j), cnt + 1);
        }
    }

    static int maxWords(String[] dict) {
        List<Long> placements = new ArrayList<>();
        for (String w : dict) {
            Set<Long> masks = new HashSet<>();
            for (int r = 0; r < R; r++)
                for (int c = 0; c < C; c++) {
                    List<Long> tmp = new ArrayList<>();
                    findWord(w, 0, r, c, 0L, tmp);
                    masks.addAll(tmp);
                }
            placements.addAll(masks);
        }
        best = 0;
        pack(placements, 0, 0L, 0);
        return best;
    }

    public static void main(String[] args) {
        board = new char[][]{{'e', 'a', 'n'}, {'t', 't', 'i'}, {'a', 'r', 'a'}};
        R = board.length; C = board[0].length;
        String[] dict = {"eat", "rain", "in", "rat"};
        System.out.println(maxWords(dict)); // 3
    }
}
