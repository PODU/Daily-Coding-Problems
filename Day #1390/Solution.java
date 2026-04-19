// Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.
public class Solution {
    public static void main(String[] args) {
        int[][] m = {
            {0, 3, 1, 1},
            {2, 0, 0, 4},
            {1, 5, 3, 1}
        };
        int R = m.length, C = m[0].length;
        int[][] dp = new int[R][C];
        for (int i = 0; i < R; i++)
            for (int j = 0; j < C; j++) {
                int best = 0;
                if (i > 0) best = Math.max(best, dp[i-1][j]);
                if (j > 0) best = Math.max(best, dp[i][j-1]);
                dp[i][j] = m[i][j] + ((i == 0 && j == 0) ? 0 : best);
            }
        System.out.println(dp[R-1][C-1]); // 12
    }
}
