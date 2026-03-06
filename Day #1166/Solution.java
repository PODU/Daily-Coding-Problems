// Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
// Time: O(rows*cols), Space: O(rows*cols).
import java.util.ArrayDeque;

public class Solution {
    static void floodFill(char[][] g, int sr, int sc, char color) {
        int rows = g.length, cols = g[0].length;
        char start = g[sr][sc];
        if (start == color) return;
        ArrayDeque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{sr, sc});
        g[sr][sc] = color;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            for (int d = 0; d < 4; d++) {
                int nr = cur[0] + dr[d], nc = cur[1] + dc[d];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] == start) {
                    g[nr][nc] = color;
                    q.add(new int[]{nr, nc});
                }
            }
        }
    }

    public static void main(String[] args) {
        char[][] g = {
            {'B','B','W'},
            {'W','W','W'},
            {'W','W','W'},
            {'B','B','B'}
        };
        floodFill(g, 2, 2, 'G');
        StringBuilder sb = new StringBuilder();
        for (char[] row : g) {
            for (int j = 0; j < row.length; j++) {
                if (j > 0) sb.append(' ');
                sb.append(row[j]);
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
