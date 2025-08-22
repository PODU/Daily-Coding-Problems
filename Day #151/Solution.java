// Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
// region with new color. Time O(R*C), Space O(R*C).
import java.util.*;

public class Solution {
    static void floodFill(char[][] m, int r, int c, char color) {
        int R = m.length, C = m[0].length;
        char target = m[r][c];
        if (target == color) return;
        Deque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{r, c});
        m[r][c] = color;
        int[] dr = {-1,1,0,0}, dc = {0,0,-1,1};
        while (!q.isEmpty()) {
            int[] cur = q.poll();
            for (int d = 0; d < 4; d++) {
                int nx = cur[0]+dr[d], ny = cur[1]+dc[d];
                if (nx>=0 && nx<R && ny>=0 && ny<C && m[nx][ny]==target) {
                    m[nx][ny] = color;
                    q.add(new int[]{nx, ny});
                }
            }
        }
    }

    public static void main(String[] args) {
        char[][] m = {
            {'B','B','W'},
            {'W','W','W'},
            {'W','W','W'},
            {'B','B','B'}
        };
        floodFill(m, 2, 2, 'G');
        StringBuilder sb = new StringBuilder();
        for (char[] row : m) {
            for (int j = 0; j < row.length; j++) {
                sb.append(row[j]);
                if (j+1 < row.length) sb.append(' ');
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
