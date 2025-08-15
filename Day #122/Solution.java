// Day 122: Max coins from top-left to bottom-right moving right/down.
// DP O(R*C) time and space, with path reconstruction (prefer left on ties).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] g = {{0, 3, 1, 1}, {2, 0, 0, 4}, {1, 5, 3, 1}};
        int R = g.length, C = g[0].length;
        int[][] dp = new int[R][C];
        for (int i = 0; i < R; i++)
            for (int j = 0; j < C; j++) {
                int best = 0;
                if (i == 0 && j == 0) best = 0;
                else if (i == 0) best = dp[i][j - 1];
                else if (j == 0) best = dp[i - 1][j];
                else best = Math.max(dp[i - 1][j], dp[i][j - 1]);
                dp[i][j] = g[i][j] + best;
            }
        List<Integer> path = new ArrayList<>();
        int i = R - 1, j = C - 1;
        while (i > 0 || j > 0) {
            path.add(g[i][j]);
            if (i == 0) j--;
            else if (j == 0) i--;
            else if (dp[i - 1][j] > dp[i][j - 1]) i--;
            else j--;
        }
        path.add(g[0][0]);
        Collections.reverse(path);
        StringBuilder sb = new StringBuilder();
        for (int t = 0; t < path.size(); t++) {
            if (t > 0) sb.append(" + ");
            sb.append(path.get(t));
        }
        System.out.println("The most we can collect is " + sb + " = " + dp[R - 1][C - 1] + " coins.");
    }
}
