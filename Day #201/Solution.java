// Day 201: Maximum weight path in a triangle.
// Bottom-up DP: each cell becomes its value + max of the two children below.
// Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    static int maxPath(int[][] t) {
        int n = t.length;
        int[] dp = Arrays.copyOf(t[n - 1], t[n - 1].length);
        for (int r = n - 2; r >= 0; r--)
            for (int c = 0; c <= r; c++)
                dp[c] = t[r][c] + Math.max(dp[c], dp[c + 1]);
        return dp[0];
    }

    public static void main(String[] args) {
        int[][] t = {{1}, {2, 3}, {1, 5, 1}};
        System.out.println(maxPath(t)); // 9
    }
}
