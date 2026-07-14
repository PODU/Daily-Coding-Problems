// Knight-on-board probability after k random moves: DP over board states.
// dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
// Time: O(k*64*8). Space: O(64).
public class Solution {
    static final int[] DR = {-2,-2,-1,-1,1,1,2,2};
    static final int[] DC = {-1,1,-2,2,-2,2,-1,1};

    static double knightProbability(int N, int k, int r0, int c0) {
        double[][] dp = new double[N][N];
        dp[r0][c0] = 1.0;
        for (int m = 0; m < k; m++) {
            double[][] nxt = new double[N][N];
            for (int r = 0; r < N; r++)
                for (int c = 0; c < N; c++) {
                    if (dp[r][c] == 0) continue;
                    for (int d = 0; d < 8; d++) {
                        int nr = r + DR[d], nc = c + DC[d];
                        if (nr >= 0 && nr < N && nc >= 0 && nc < N)
                            nxt[nr][nc] += dp[r][c] / 8.0;
                    }
                }
            dp = nxt;
        }
        double total = 0;
        for (double[] row : dp) for (double v : row) total += v;
        return total;
    }

    public static void main(String[] args) {
        // corner (0,0), k=1 -> 2/8 = 0.25
        System.out.println(knightProbability(8, 1, 0, 0)); // 0.25
    }
}
