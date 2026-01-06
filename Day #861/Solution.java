// Day 861: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
// Time: O(k * N^2), Space: O(N^2).
public class Solution {
    static final int[] DR = {-2,-2,-1,-1,1,1,2,2};
    static final int[] DC = {-1,1,-2,2,-2,2,-1,1};

    static double knightProbability(int N, int k, int sr, int sc) {
        double[][] dp = new double[N][N];
        dp[sr][sc] = 1.0;
        for (int step = 0; step < k; step++) {
            double[][] nx = new double[N][N];
            for (int r = 0; r < N; r++)
                for (int c = 0; c < N; c++)
                    if (dp[r][c] > 0)
                        for (int d = 0; d < 8; d++) {
                            int nr = r + DR[d], nc = c + DC[d];
                            if (nr >= 0 && nr < N && nc >= 0 && nc < N)
                                nx[nr][nc] += dp[r][c] / 8.0;
                        }
            dp = nx;
        }
        double total = 0;
        for (double[] row : dp) for (double v : row) total += v;
        return total;
    }

    public static void main(String[] args) {
        System.out.println(knightProbability(8, 1, 0, 0));  // 0.25
        System.out.println(knightProbability(8, 2, 0, 0));
    }
}
