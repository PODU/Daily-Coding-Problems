// Shortest path on a grid with walls using BFS (4-directional).
// Time: O(M*N), Space: O(M*N).
import java.util.*;

public class Solution {
    static Integer shortestPath(boolean[][] board, int[] start, int[] end) {
        int m = board.length, n = board[0].length;
        boolean[][] visited = new boolean[m][n];
        int[] dr = {-1, 1, 0, 0};
        int[] dc = {0, 0, -1, 1};
        Queue<int[]> q = new LinkedList<>();
        q.add(new int[]{start[0], start[1], 0});
        visited[start[0]][start[1]] = true;
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            int r = cur[0], c = cur[1], d = cur[2];
            if (r == end[0] && c == end[1]) return d;
            for (int k = 0; k < 4; k++) {
                int nr = r + dr[k], nc = c + dc[k];
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !board[nr][nc]) {
                    visited[nr][nc] = true;
                    q.add(new int[]{nr, nc, d + 1});
                }
            }
        }
        return null;
    }

    public static void main(String[] args) {
        boolean f = false, t = true;
        boolean[][] board = {
            {f, f, f, f},
            {t, t, f, t},
            {f, f, f, f},
            {f, f, f, f}
        };
        Integer res = shortestPath(board, new int[]{3, 0}, new int[]{0, 0});
        System.out.println(res == null ? "None" : res);
    }
}
