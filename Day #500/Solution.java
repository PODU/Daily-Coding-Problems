// BFS shortest path on a boolean grid (false=walkable, true=wall).
// Time O(M*N), Space O(M*N).
import java.util.*;

public class Solution {
    static Integer minSteps(boolean[][] board, int[] start, int[] end) {
        int M = board.length, N = board[0].length;
        if (board[start[0]][start[1]] || board[end[0]][end[1]]) return null;
        boolean[][] visited = new boolean[M][N];
        Queue<int[]> q = new LinkedList<>();
        q.add(start);
        visited[start[0]][start[1]] = true;
        int steps = 0;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int sz = q.size();
            for (int i = 0; i < sz; i++) {
                int[] cur = q.poll();
                if (cur[0] == end[0] && cur[1] == end[1]) return steps;
                for (int d = 0; d < 4; d++) {
                    int nr = cur[0] + dr[d], nc = cur[1] + dc[d];
                    if (nr >= 0 && nr < M && nc >= 0 && nc < N && !visited[nr][nc] && !board[nr][nc]) {
                        visited[nr][nc] = true;
                        q.add(new int[]{nr, nc});
                    }
                }
            }
            steps++;
        }
        return null;
    }

    public static void main(String[] args) {
        boolean t = true, f = false;
        boolean[][] board = {
            {f, f, f, f},
            {t, t, f, t},
            {f, f, f, f},
            {f, f, f, f}
        };
        System.out.println(minSteps(board, new int[]{3, 0}, new int[]{0, 0}));
    }
}
