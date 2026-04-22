// Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
// Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).
import java.util.*;

public class Solution {
    static long countTrue(String[] expr) {
        List<Character> vals = new ArrayList<>(), ops = new ArrayList<>();
        for (int i = 0; i < expr.length; i++)
            if (i % 2 == 0) vals.add(expr[i].charAt(0));
            else ops.add(expr[i].charAt(0));
        int n = vals.size();
        if (n == 0) return 0;
        long[][] T = new long[n][n], F = new long[n][n];
        for (int i = 0; i < n; i++) {
            T[i][i] = vals.get(i) == 'T' ? 1 : 0;
            F[i][i] = vals.get(i) == 'F' ? 1 : 0;
        }
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                for (int k = i; k < j; k++) {
                    char op = ops.get(k);
                    long lt = T[i][k], lf = F[i][k], rt = T[k+1][j], rf = F[k+1][j];
                    long tot = (lt + lf) * (rt + rf), t;
                    if (op == '&') t = lt * rt;
                    else if (op == '|') t = lt * rt + lt * rf + lf * rt;
                    else t = lt * rf + lf * rt;
                    T[i][j] += t;
                    F[i][j] += tot - t;
                }
            }
        return T[0][n - 1];
    }

    public static void main(String[] args) {
        System.out.println(countTrue(new String[]{"F", "|", "T", "&", "T"})); // 2
    }
}
