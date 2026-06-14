// Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
import java.util.*;

public class Solution {
    static int maxPath(int[][] t) {
        int n = t.length;
        int[] dp = Arrays.copyOf(t[n - 1], n);
        for (int i = n - 2; i >= 0; i--)
            for (int j = 0; j <= i; j++)
                dp[j] = t[i][j] + Math.max(dp[j], dp[j + 1]);
        return dp[0];
    }

    public static void main(String[] a) {
        int[][] t = {{1}, {2, 3}, {1, 5, 1}};
        System.out.println(maxPath(t));
    }
}
