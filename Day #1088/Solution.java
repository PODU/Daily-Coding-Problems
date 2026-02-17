// Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
public class Solution {
    static long countTrue(char[] expr) {
        int m = expr.length;
        int n = (m + 1) / 2;
        boolean[] val = new boolean[n];
        char[] ops = new char[Math.max(0, n - 1)];
        for (int i = 0; i < m; i++) {
            if (i % 2 == 0) val[i / 2] = (expr[i] == 'T');
            else ops[i / 2] = expr[i];
        }
        long[][] T = new long[n][n], F = new long[n][n];
        for (int i = 0; i < n; i++) { T[i][i] = val[i] ? 1 : 0; F[i][i] = val[i] ? 0 : 1; }
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                for (int k = i; k < j; k++) {
                    char op = ops[k];
                    long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
                    if (op == '&') { T[i][j] += lt * rt; F[i][j] += lf * rf + lf * rt + lt * rf; }
                    else if (op == '|') { T[i][j] += lt * rt + lt * rf + lf * rt; F[i][j] += lf * rf; }
                    else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
                }
            }
        return T[0][n - 1];
    }

    public static void main(String[] a) {
        char[] expr = {'F', '|', 'T', '&', 'T'};
        System.out.println(countTrue(expr));
    }
}
