// Day 1305: Longest common subsequence of three strings.
// 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.
public class Solution {
    static int lcs3(String a, String b, String c) {
        int n = a.length(), m = b.length(), p = c.length();
        int[][][] dp = new int[n + 1][m + 1][p + 1];
        for (int i = 1; i <= n; i++)
            for (int j = 1; j <= m; j++)
                for (int k = 1; k <= p; k++) {
                    if (a.charAt(i - 1) == b.charAt(j - 1) && b.charAt(j - 1) == c.charAt(k - 1))
                        dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                    else
                        dp[i][j][k] = Math.max(dp[i - 1][j][k],
                                Math.max(dp[i][j - 1][k], dp[i][j][k - 1]));
                }
        return dp[n][m][p];
    }

    public static void main(String[] args) {
        System.out.println(lcs3("epidemiologist", "refrigeration",
                "supercalifragilisticexpialodocious")); // 5
    }
}
