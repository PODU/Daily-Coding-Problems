// Day 1762: Count parenthesizations of a boolean expression evaluating to True.
// Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
// combine across each split operator. Time O(n^3), Space O(n^2).
import java.util.*;

public class Solution {
    static long countTrue(String[] tokens) {
        List<Character> vals = new ArrayList<>(), ops = new ArrayList<>();
        for (int i = 0; i < tokens.length; i++)
            (i % 2 == 0 ? vals : ops).add(tokens[i].charAt(0));
        int n = vals.size();
        long[][] t = new long[n][n], f = new long[n][n];
        for (int i = 0; i < n; i++) {
            t[i][i] = vals.get(i) == 'T' ? 1 : 0;
            f[i][i] = vals.get(i) == 'F' ? 1 : 0;
        }
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                for (int k = i; k < j; k++) {
                    char op = ops.get(k);
                    long lt = t[i][k], lf = f[i][k], rt = t[k + 1][j], rf = f[k + 1][j];
                    long tot = (lt + lf) * (rt + rf);
                    if (op == '&') { t[i][j] += lt * rt; f[i][j] += tot - lt * rt; }
                    else if (op == '|') { f[i][j] += lf * rf; t[i][j] += tot - lf * rf; }
                    else { t[i][j] += lt * rf + lf * rt; f[i][j] += lt * rt + lf * rf; }
                }
            }
        return t[0][n - 1];
    }

    public static void main(String[] args) {
        String[] tokens = {"F", "|", "T", "&", "T"};
        System.out.println(countTrue(tokens));
    }
}
