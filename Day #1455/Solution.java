// Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
// each row into the one above. Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static int maxPathSum(int[][] triangle) {
        if (triangle.length == 0) return 0;
        int n = triangle.length;
        int[] dp = Arrays.copyOf(triangle[n - 1], triangle[n - 1].length);
        for (int r = n - 2; r >= 0; r--)
            for (int i = 0; i <= r; i++)
                dp[i] = triangle[r][i] + Math.max(dp[i], dp[i + 1]);
        return dp[0];
    }

    public static void main(String[] args) {
        int[][] triangle = {{1}, {2, 3}, {1, 5, 1}};
        System.out.println(maxPathSum(triangle)); // 9  (1 -> 3 -> 5)
    }
}
