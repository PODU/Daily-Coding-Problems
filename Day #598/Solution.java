// Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
// DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n^2).
public class Solution {
    static double expectedRounds(int n) {
        if (n <= 1) return 0.0;
        double[][] C = new double[n + 1][n + 1];
        for (int i = 0; i <= n; i++) {
            C[i][0] = 1.0;
            for (int j = 1; j <= i; j++)
                C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
        }
        double[] f = new double[n + 1];
        for (int m = 2; m <= n; m++) {
            double half = Math.pow(0.5, m);
            double s = 0.0;
            for (int k = 0; k < m; k++) s += C[m][k] * f[k];
            f[m] = (1.0 + half * s) / (1.0 - half);
        }
        return f[n];
    }

    public static void main(String[] args) {
        int n = 4;
        System.out.printf("%.4f%n", expectedRounds(n)); // ~2.0571
    }
}
