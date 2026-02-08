// LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n1*n2*n3).
public class Solution {
    static int lcs3(String a, String b, String c) {
        int n1 = a.length(), n2 = b.length(), n3 = c.length();
        int[][][] dp = new int[n1 + 1][n2 + 1][n3 + 1];
        for (int i = 1; i <= n1; i++)
            for (int j = 1; j <= n2; j++)
                for (int k = 1; k <= n3; k++) {
                    if (a.charAt(i - 1) == b.charAt(j - 1) && b.charAt(j - 1) == c.charAt(k - 1))
                        dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                    else
                        dp[i][j][k] = Math.max(dp[i - 1][j][k],
                                Math.max(dp[i][j - 1][k], dp[i][j][k - 1]));
                }
        return dp[n1][n2][n3];
    }

    public static void main(String[] args) {
        System.out.println(lcs3("epidemiologist", "refrigeration",
                "supercalifragilisticexpialodocious"));
    }
}
