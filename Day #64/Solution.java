// Count open knight's tours on N x N board via backtracking DFS from every start.
// Time exponential, Space O(N^2).
public class Solution {
    static int N;
    static final int[] DR = {-2,-2,-1,-1,1,1,2,2};
    static final int[] DC = {-1,1,-2,2,-2,2,-1,1};

    static long dfs(boolean[][] vis, int r, int c, int count) {
        if (count == N * N) return 1;
        long total = 0;
        for (int k = 0; k < 8; ++k) {
            int nr = r + DR[k], nc = c + DC[k];
            if (nr >= 0 && nr < N && nc >= 0 && nc < N && !vis[nr][nc]) {
                vis[nr][nc] = true;
                total += dfs(vis, nr, nc, count + 1);
                vis[nr][nc] = false;
            }
        }
        return total;
    }

    static long countTours(int n) {
        N = n;
        long total = 0;
        for (int r = 0; r < n; ++r)
            for (int c = 0; c < n; ++c) {
                boolean[][] vis = new boolean[n][n];
                vis[r][c] = true;
                total += dfs(vis, r, c, 1);
            }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(countTours(5));
    }
}
