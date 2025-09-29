// Min perfect squares summing to N via DP, then greedy-largest reconstruction.
// Time O(N*sqrt N), Space O(N).
import java.util.*;

public class Solution {
    static String solve(int n) {
        int[] dp = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int i = 1; i <= n; i++)
            for (int s = 1; s * s <= i; s++)
                dp[i] = Math.min(dp[i], dp[i - s * s] + 1);

        // Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
        List<Integer> squares = new ArrayList<>();
        int i = n;
        while (i > 0) {
            for (int s = (int) Math.sqrt(i); s >= 1; s--) {
                if (s * s <= i && dp[i - s * s] == dp[i] - 1) {
                    squares.add(s * s);
                    i -= s * s;
                    break;
                }
            }
        }

        StringBuilder out = new StringBuilder();
        out.append(dp[n]).append(" (");
        for (int k = 0; k < squares.size(); k++) {
            if (k > 0) out.append(" + ");
            out.append(squares.get(k));
        }
        out.append(")");
        return out.toString();
    }

    public static void main(String[] args) {
        for (int n : new int[]{4, 17, 18}) System.out.println(solve(n));
    }
}
