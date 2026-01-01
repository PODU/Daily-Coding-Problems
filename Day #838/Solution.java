// Day 838: Max coins moving only right/down through a grid.
// DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).
public class Solution {
    static int maxCoins(int[][] grid) {
        if (grid.length == 0 || grid[0].length == 0) return 0;
        int rows = grid.length, cols = grid[0].length;
        int[][] dp = new int[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                int best = 0;
                if (i > 0) best = Math.max(best, dp[i - 1][j]);
                if (j > 0) best = Math.max(best, dp[i][j - 1]);
                dp[i][j] = grid[i][j] + best;
            }
        }
        return dp[rows - 1][cols - 1];
    }

    public static void main(String[] args) {
        int[][] matrix = {
            {0, 3, 1, 1},
            {2, 0, 0, 4},
            {1, 5, 3, 1},
        };
        System.out.println(maxCoins(matrix));
    }
}
