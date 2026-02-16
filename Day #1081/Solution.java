// Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
import java.util.*;

public class Solution {
    static void floodFill(char[][] img, int sr, int sc, char c) {
        char src = img[sr][sc];
        if (src == c) return;
        int R = img.length, C = img[0].length;
        Deque<int[]> q = new ArrayDeque<>(); q.add(new int[]{sr, sc}); img[sr][sc] = c;
        int[] dx = {0,0,1,-1}, dy = {1,-1,0,0};
        while (!q.isEmpty()) {
            int[] p = q.poll();
            for (int d = 0; d < 4; d++) {
                int nr = p[0] + dx[d], nc = p[1] + dy[d];
                if (nr >= 0 && nr < R && nc >= 0 && nc < C && img[nr][nc] == src) {
                    img[nr][nc] = c; q.add(new int[]{nr, nc});
                }
            }
        }
    }

    public static void main(String[] a) {
        char[][] img = {{'B','B','W'},{'W','W','W'},{'W','W','W'},{'B','B','B'}};
        floodFill(img, 2, 2, 'G');
        StringBuilder sb = new StringBuilder();
        for (char[] row : img) {
            for (int i = 0; i < row.length; i++) { sb.append(row[i]); if (i + 1 < row.length) sb.append(' '); }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
