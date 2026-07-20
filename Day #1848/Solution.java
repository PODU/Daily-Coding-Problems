// Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
// Time O(L1*L2*L3), Space O(L1*L2*L3).
public class Solution {
    static int lcs3(String a, String b, String c) {
        int la = a.length(), lb = b.length(), lc = c.length();
        int[][][] dp = new int[la + 1][lb + 1][lc + 1];
        for (int i = 1; i <= la; i++)
            for (int j = 1; j <= lb; j++)
                for (int k = 1; k <= lc; k++) {
                    if (a.charAt(i - 1) == b.charAt(j - 1) && b.charAt(j - 1) == c.charAt(k - 1))
                        dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                    else
                        dp[i][j][k] = Math.max(dp[i - 1][j][k],
                                Math.max(dp[i][j - 1][k], dp[i][j][k - 1]));
                }
        return dp[la][lb][lc];
    }

    public static void main(String[] args) {
        System.out.println(lcs3("epidemiologist", "refrigeration",
                "supercalifragilisticexpialodocious")); // 5
    }
}
