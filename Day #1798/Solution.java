// Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
// Walls are booleans (true=wall). Returns -1 if no path (null semantics).
import java.util.*;

public class Solution {
    static int shortestPath(boolean[][] grid, int[] start, int[] end) {
        int m = grid.length, n = grid[0].length;
        if (grid[start[0]][start[1]] || grid[end[0]][end[1]]) return -1;
        boolean[][] visited = new boolean[m][n];
        Queue<int[]> q = new LinkedList<>();
        q.add(start);
        visited[start[0]][start[1]] = true;
        int steps = 0;
        int[] dr = {-1, 1, 0, 0};
        int[] dc = {0, 0, -1, 1};
        while (!q.isEmpty()) {
            int sz = q.size();
            for (int i = 0; i < sz; i++) {
                int[] cur = q.poll();
                if (cur[0] == end[0] && cur[1] == end[1]) return steps;
                for (int d = 0; d < 4; d++) {
                    int nr = cur[0] + dr[d], nc = cur[1] + dc[d];
                    if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !grid[nr][nc]) {
                        visited[nr][nc] = true;
                        q.add(new int[]{nr, nc});
                    }
                }
            }
            steps++;
        }
        return -1;
    }

    public static void main(String[] args) {
        boolean F = false, T = true;
        boolean[][] grid = {
            {F, F, F, F},
            {T, T, F, T},
            {F, F, F, F},
            {F, F, F, F}
        };
        System.out.println(shortestPath(grid, new int[]{3, 0}, new int[]{0, 0})); // 7
    }
}
