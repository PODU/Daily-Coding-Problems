// Boolean parenthesization: count ways the expression evaluates to True.
// Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).
public class Solution {
    static long countTrue(String[] a) {
        int n = a.length, M = (n + 1) / 2;
        char[] val = new char[M];
        char[] op = new char[M - 1];
        for (int i = 0, vi = 0, oi = 0; i < n; i++) {
            if (i % 2 == 0) val[vi++] = a[i].charAt(0);
            else op[oi++] = a[i].charAt(0);
        }
        long[][] T = new long[M][M], F = new long[M][M];
        for (int i = 0; i < M; i++) { T[i][i] = val[i] == 'T' ? 1 : 0; F[i][i] = val[i] == 'F' ? 1 : 0; }
        for (int len = 2; len <= M; len++)
            for (int i = 0; i + len - 1 < M; i++) {
                int j = i + len - 1;
                for (int k = i; k < j; k++) {
                    char o = op[k];
                    long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
                    long tot = (lt + lf) * (rt + rf);
                    if (o == '&') { T[i][j] += lt * rt; F[i][j] += tot - lt * rt; }
                    else if (o == '|') { T[i][j] += tot - lf * rf; F[i][j] += lf * rf; }
                    else { T[i][j] += lt * rf + lf * rt; F[i][j] += lt * rt + lf * rf; }
                }
            }
        return T[0][M - 1];
    }

    public static void main(String[] args) {
        String[] expr = {"F", "|", "T", "&", "T"};
        System.out.println(countTrue(expr));
    }
}
