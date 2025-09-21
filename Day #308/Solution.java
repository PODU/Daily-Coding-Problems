// Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
public class Solution {
    static long countTrue(String[] e) {
        int n = e.length, V = (n + 1) / 2;
        long[][] T = new long[V][V], F = new long[V][V];
        for (int i = 0; i < V; i++) {
            boolean val = e[2 * i].equals("T");
            T[i][i] = val ? 1 : 0; F[i][i] = val ? 0 : 1;
        }
        for (int len = 2; len <= V; len++) for (int i = 0; i + len - 1 < V; i++) {
            int j = i + len - 1;
            for (int k = i; k < j; k++) {
                String op = e[2 * k + 1];
                long lt = T[i][k], lf = F[i][k], rt = T[k + 1][j], rf = F[k + 1][j];
                long total = (lt + lf) * (rt + rf), t = 0;
                if (op.equals("&")) t = lt * rt;
                else if (op.equals("|")) t = lt * rt + lt * rf + lf * rt;
                else t = lt * rf + lf * rt;
                T[i][j] += t; F[i][j] += total - t;
            }
        }
        return T[0][V - 1];
    }
    public static void main(String[] a) {
        String[] e = {"F", "|", "T", "&", "T"};
        System.out.println(countTrue(e)); // 2
    }
}
