// Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
public class Solution {
    static int maxCoins(int[][] grid) {
        int R = grid.length, C = grid[0].length;
        int[][] dp = new int[R][C];
        for (int i = 0; i < R; i++)
            for (int j = 0; j < C; j++) {
                int best = 0;
                if (i > 0) best = Math.max(best, dp[i-1][j]);
                if (j > 0) best = Math.max(best, dp[i][j-1]);
                dp[i][j] = grid[i][j] + ((i == 0 && j == 0) ? 0 : best);
            }
        return dp[R-1][C-1];
    }

    public static void main(String[] args) {
        int[][] grid = {{0,3,1,1},{2,0,0,4},{1,5,3,1}};
        int result = maxCoins(grid);
        assert result == 12;
        if (result != 12) throw new AssertionError("expected 12");
        System.out.println("The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = " + result + " coins.");
    }
}
