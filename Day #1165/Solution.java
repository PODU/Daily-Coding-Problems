// Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
// Time: O(k*64*8), Space: O(64).
public class Solution {
    static double knightProbability(int k, int startR, int startC) {
        int[] dr = {-2,-2,-1,-1,1,1,2,2};
        int[] dc = {-1,1,-2,2,-2,2,-1,1};
        double[][] prob = new double[8][8];
        for (double[] row : prob) java.util.Arrays.fill(row, 1.0);
        for (int m = 1; m <= k; m++) {
            double[][] next = new double[8][8];
            for (int r = 0; r < 8; r++)
                for (int c = 0; c < 8; c++) {
                    double s = 0.0;
                    for (int d = 0; d < 8; d++) {
                        int nr = r + dr[d], nc = c + dc[d];
                        if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8)
                            s += prob[nr][nc];
                    }
                    next[r][c] = s / 8.0;
                }
            prob = next;
        }
        return prob[startR][startC];
    }

    public static void main(String[] args) {
        double ans = knightProbability(1, 0, 0);
        String out = new java.math.BigDecimal(ans)
            .setScale(2, java.math.RoundingMode.HALF_UP)
            .stripTrailingZeros().toPlainString();
        System.out.println(out);
    }
}
