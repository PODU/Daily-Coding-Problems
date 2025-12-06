// Day 703: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over board cells; each move spreads prob/8 to valid targets.
// Time O(k * N^2 * 8), Space O(N^2).
public class Solution {
    static double knightProbability(int N, int k, int r, int c) {
        int[] dr = {1, 1, -1, -1, 2, 2, -2, -2}, dc = {2, -2, 2, -2, 1, -1, 1, -1};
        double[][] dp = new double[N][N];
        dp[r][c] = 1.0;
        for (int step = 0; step < k; step++) {
            double[][] nd = new double[N][N];
            for (int i = 0; i < N; i++)
                for (int j = 0; j < N; j++)
                    if (dp[i][j] > 0)
                        for (int m = 0; m < 8; m++) {
                            int ni = i + dr[m], nj = j + dc[m];
                            if (ni >= 0 && ni < N && nj >= 0 && nj < N)
                                nd[ni][nj] += dp[i][j] / 8.0;
                        }
            dp = nd;
        }
        double total = 0;
        for (double[] row : dp) for (double v : row) total += v;
        return total;
    }

    public static void main(String[] args) {
        System.out.println(knightProbability(8, 2, 0, 0)); // 0.1875
    }
}
