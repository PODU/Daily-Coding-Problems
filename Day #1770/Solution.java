// Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
// Time: O(rows*cols), Space: O(rows*cols) for the queue in worst case.
import java.util.*;

public class Solution {
    static void floodFill(char[][] g, int sr, int sc, char color) {
        int R = g.length, C = g[0].length;
        char target = g[sr][sc];
        if (target == color) return;
        Queue<int[]> q = new LinkedList<>();
        q.add(new int[]{sr, sc});
        g[sr][sc] = color;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            for (int k = 0; k < 4; k++) {
                int nr = cur[0] + dr[k], nc = cur[1] + dc[k];
                if (nr >= 0 && nr < R && nc >= 0 && nc < C && g[nr][nc] == target) {
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
                sb.append(row[j]);
                if (j + 1 < row.length) sb.append(' ');
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
