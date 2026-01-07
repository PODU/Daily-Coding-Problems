// Day 867: Maximum weight path from top to bottom of a triangle.
// Approach: bottom-up DP, fold each row into the one above using max of adjacent.
// Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    static int maxPath(int[][] t) {
        int n = t.length;
        int[] dp = Arrays.copyOf(t[n - 1], t[n - 1].length);
        for (int i = n - 2; i >= 0; i--)
            for (int j = 0; j <= i; j++)
                dp[j] = t[i][j] + Math.max(dp[j], dp[j + 1]);
        return dp[0];
    }

    public static void main(String[] args) {
        int[][] t = {{1}, {2, 3}, {1, 5, 1}};
        System.out.println(maxPath(t)); // 9
    }
}
