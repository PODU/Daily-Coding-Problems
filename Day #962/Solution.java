// Day 962: Count knight's tours on an N x N board (visit every square once).
// Approach: DFS backtracking from every start square. Time O(8^(N^2)) worst, Space O(N^2).
public class Solution {
    static int N;
    static int[] dx = {1,1,-1,-1,2,2,-2,-2};
    static int[] dy = {2,-2,2,-2,1,-1,1,-1};

    static long dfs(boolean[][] vis, int x, int y, int count) {
        if (count == N * N) return 1;
        long total = 0;
        for (int d = 0; d < 8; d++) {
            int nx = x + dx[d], ny = y + dy[d];
            if (nx >= 0 && nx < N && ny >= 0 && ny < N && !vis[nx][ny]) {
                vis[nx][ny] = true;
                total += dfs(vis, nx, ny, count + 1);
                vis[nx][ny] = false;
            }
        }
        return total;
    }

    static long countTours(int n) {
        N = n;
        long total = 0;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                boolean[][] vis = new boolean[n][n];
                vis[i][j] = true;
                total += dfs(vis, i, j, 1);
            }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countTours(5)); // 1728
    }
}
