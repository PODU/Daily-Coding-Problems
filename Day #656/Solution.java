// Flood fill via BFS from start pixel; recolor only cells equal to original color.
// Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static void floodFill(char[][] img, int sr, int sc, char color) {
        int rows = img.length, cols = img[0].length;
        char orig = img[sr][sc];
        if (orig == color) return;
        Deque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{sr, sc});
        img[sr][sc] = color;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int[] cell = q.poll();
            for (int d = 0; d < 4; d++) {
                int nr = cell[0] + dr[d], nc = cell[1] + dc[d];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && img[nr][nc] == orig) {
                    img[nr][nc] = color;
                    q.add(new int[]{nr, nc});
                }
            }
        }
    }

    public static void main(String[] args) {
        char[][] img = {
            {'B', 'B', 'W'},
            {'W', 'W', 'W'},
            {'W', 'W', 'W'},
            {'B', 'B', 'B'}};
        floodFill(img, 2, 2, 'G');
        StringBuilder sb = new StringBuilder();
        for (char[] row : img) {
            for (int j = 0; j < row.length; j++) {
                sb.append(row[j]);
                if (j + 1 < row.length) sb.append(' ');
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
