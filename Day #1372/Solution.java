// Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
// Returns null (printed) if unreachable.
import java.util.*;

public class Solution {
    static Integer shortestPath(boolean[][] g, int[] start, int[] end) {
        int m = g.length, n = g[0].length;
        if (g[start[0]][start[1]] || g[end[0]][end[1]]) return null;
        int[][] dist = new int[m][n];
        for (int[] row : dist) Arrays.fill(row, -1);
        Deque<int[]> q = new ArrayDeque<>();
        q.add(start); dist[start[0]][start[1]] = 0;
        int[] dx = {-1, 1, 0, 0}, dy = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            int r = cur[0], c = cur[1];
            if (r == end[0] && c == end[1]) return dist[r][c];
            for (int k = 0; k < 4; k++) {
                int nr = r + dx[k], nc = c + dy[k];
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && !g[nr][nc] && dist[nr][nc] == -1) {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.add(new int[]{nr, nc});
                }
            }
        }
        return null;
    }

    public static void main(String[] args) {
        boolean t = true, f = false;
        boolean[][] g = {
            {f, f, f, f},
            {t, t, f, t},
            {f, f, f, f},
            {f, f, f, f}
        };
        Integer res = shortestPath(g, new int[]{3, 0}, new int[]{0, 0});
        System.out.println(res == null ? "null" : res);
    }
}
