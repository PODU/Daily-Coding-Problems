// Day 124: Expected flipping rounds until one coin remains.
// DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.
public class Solution {
    static double expectedRounds(int n) {
        double[] E = new double[n + 1];
        for (int m = 2; m <= n; m++) {
            double p = Math.pow(0.5, m); // p_0
            double s = 0.0;
            for (int k = 0; k < m; k++) {
                s += p * E[k];
                p = p * (m - k) / (k + 1);
            }
            double pn = Math.pow(0.5, m);
            E[m] = (1.0 + s) / (1.0 - pn);
        }
        return n <= 1 ? 0.0 : E[n];
    }

    public static void main(String[] args) {
        int[] ns = {1, 2, 3, 4, 5, 10};
        for (int n : ns)
            System.out.printf("n=%-2d expected rounds = %.6f%n", n, expectedRounds(n));
    }
}
