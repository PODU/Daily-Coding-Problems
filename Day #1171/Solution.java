// Day 1171: Validate an American-style crossword grid.
// Checks rotational symmetry, white-square connectivity (BFS), and that every
// maximal horizontal/vertical white run has length >= 3.
// Time O(N^2), Space O(N^2).  '#' = black, '.' = white.
import java.util.*;

public class Solution {
    static boolean isValidCrossword(String[] g) {
        int n = g.length;
        if (n == 0) return false;
        for (String r : g) if (r.length() != n) return false;

        // 1. Rotational symmetry.
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if ((g[i].charAt(j) == '.') != (g[n-1-i].charAt(n-1-j) == '.')) return false;

        // 2. Horizontal white runs >= 3.
        for (int i = 0; i < n; i++) {
            int run = 0;
            for (int j = 0; j <= n; j++) {
                if (j < n && g[i].charAt(j) == '.') run++;
                else { if (run > 0 && run < 3) return false; run = 0; }
            }
        }
        // 3. Vertical white runs >= 3.
        for (int j = 0; j < n; j++) {
            int run = 0;
            for (int i = 0; i <= n; i++) {
                if (i < n && g[i].charAt(j) == '.') run++;
                else { if (run > 0 && run < 3) return false; run = 0; }
            }
        }

        // 4. Connectivity.
        int total = 0, si = -1, sj = -1;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i].charAt(j) == '.') { total++; if (si < 0) { si = i; sj = j; } }
        if (total == 0) return true;
        boolean[][] vis = new boolean[n][n];
        Deque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{si, sj}); vis[si][sj] = true; int seen = 1;
        int[] dx = {0,0,1,-1}, dy = {1,-1,0,0};
        while (!q.isEmpty()) {
            int[] c = q.poll();
            for (int d = 0; d < 4; d++) {
                int nx = c[0] + dx[d], ny = c[1] + dy[d];
                if (nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx][ny] && g[nx].charAt(ny) == '.') {
                    vis[nx][ny] = true; seen++; q.add(new int[]{nx, ny});
                }
            }
        }
        return seen == total;
    }

    public static void main(String[] args) {
        String[] g1 = {".....", ".....", ".....", ".....", "....."};
        String[] g2 = {".#...", ".....", ".....", ".....", "...#."};
        System.out.println(isValidCrossword(g1) ? "true" : "false");
        System.out.println(isValidCrossword(g2) ? "true" : "false");
    }
}
