// Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
// Time O(N^2), Space O(N^2).
import java.util.ArrayDeque;

public class Solution {
    static boolean isValidCrossword(int[][] g) {
        int n = g.length;
        if (n == 0) return false;

        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i][j] != g[n - 1 - i][n - 1 - j]) return false;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (g[i][j] != 0) continue;
                int l = j, r = j;
                while (l - 1 >= 0 && g[i][l - 1] == 0) l--;
                while (r + 1 < n && g[i][r + 1] == 0) r++;
                if (r - l + 1 < 3) return false;
                int t = i, b = i;
                while (t - 1 >= 0 && g[t - 1][j] == 0) t--;
                while (b + 1 < n && g[b + 1][j] == 0) b++;
                if (b - t + 1 < 3) return false;
            }
        }

        int total = 0, start = -1;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i][j] == 0) { total++; if (start < 0) start = i * n + j; }
        if (total == 0) return false;

        boolean[][] vis = new boolean[n][n];
        ArrayDeque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{start / n, start % n});
        vis[start / n][start % n] = true;
        int seen = 0;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            seen++;
            for (int d = 0; d < 4; d++) {
                int nr = cur[0] + dr[d], nc = cur[1] + dc[d];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] && g[nr][nc] == 0) {
                    vis[nr][nc] = true;
                    q.add(new int[]{nr, nc});
                }
            }
        }
        return seen == total;
    }

    public static void main(String[] args) {
        int[][] valid = new int[5][5]; // all white (0)
        System.out.println(isValidCrossword(valid) ? "true" : "false");

        int[][] invalid = new int[5][5];
        invalid[0][0] = 1; // breaks symmetry
        System.out.println(isValidCrossword(invalid) ? "true" : "false");
    }
}
