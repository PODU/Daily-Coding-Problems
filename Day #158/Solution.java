// Day 158: Count paths (right/down only) avoiding walls. DP: dp[i][j] = ways
// into a free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).
public class Solution {
    static long countPaths(int[][] grid) {
        int n = grid.length, m = grid[0].length;
        long[] dp = new long[m];
        dp[0] = grid[0][0] == 0 ? 1 : 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == 1) { dp[j] = 0; continue; }
                if (j > 0) dp[j] += dp[j - 1];
            }
        }
        return dp[m - 1];
    }

    public static void main(String[] args) {
        int[][] grid = {
            {0, 0, 1},
            {0, 0, 1},
            {1, 0, 0}
        };
        System.out.println(countPaths(grid)); // 2
    }
}
