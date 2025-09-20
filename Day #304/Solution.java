// Day 304: Probability knight stays on board after k random moves. DP over board.
// Time O(k*N^2*8), space O(N^2).
public class Solution {
    static double knightProb(int N, int k, int sr, int sc) {
        double[][] dp = new double[N][N];
        dp[sr][sc] = 1.0;
        int[] dr = {1,1,-1,-1,2,2,-2,-2}, dc = {2,-2,2,-2,1,-1,1,-1};
        for (int step = 0; step < k; step++) {
            double[][] nd = new double[N][N];
            for (int r = 0; r < N; r++) for (int c = 0; c < N; c++) if (dp[r][c] > 0)
                for (int d = 0; d < 8; d++) {
                    int nr = r + dr[d], nc = c + dc[d];
                    if (nr >= 0 && nr < N && nc >= 0 && nc < N) nd[nr][nc] += dp[r][c] / 8.0;
                }
            dp = nd;
        }
        double tot = 0;
        for (double[] row : dp) for (double v : row) tot += v;
        return tot;
    }
    public static void main(String[] a) {
        System.out.println(knightProb(8, 1, 0, 0)); // 0.25
    }
}
