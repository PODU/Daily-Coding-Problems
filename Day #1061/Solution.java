// Shortest path on a boolean grid (true=wall) via BFS in 4 directions.
// Time: O(M*N), Space: O(M*N). Returns null if unreachable.
import java.util.*;

public class Solution {
    static Integer shortestPath(boolean[][] board, int[] start, int[] end) {
        int m = board.length, n = board[0].length;
        if (board[start[0]][start[1]] || board[end[0]][end[1]]) return null;
        boolean[][] seen = new boolean[m][n];
        Queue<int[]> q = new ArrayDeque<>();
        q.add(start);
        seen[start[0]][start[1]] = true;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        int steps = 0;
        while (!q.isEmpty()) {
            int sz = q.size();
            for (int i = 0; i < sz; i++) {
                int[] cur = q.poll();
                if (cur[0] == end[0] && cur[1] == end[1]) return steps;
                for (int d = 0; d < 4; d++) {
                    int nr = cur[0] + dr[d], nc = cur[1] + dc[d];
                    if (nr < 0 || nr >= m || nc < 0 || nc >= n) continue;
                    if (seen[nr][nc] || board[nr][nc]) continue;
                    seen[nr][nc] = true;
                    q.add(new int[]{nr, nc});
                }
            }
            steps++;
        }
        return null;
    }

    public static void main(String[] args) {
        boolean[][] board = {
            {false, false, false, false},
            {true,  true,  false, true },
            {false, false, false, false},
            {false, false, false, false}
        };
        Integer res = shortestPath(board, new int[]{3, 0}, new int[]{0, 0});
        System.out.println(res == null ? "null" : res);
    }
}
