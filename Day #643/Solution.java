// Day 643: Longest common subsequence of three strings.
// Approach: 3D DP over prefix lengths of a, b, c.
// Time: O(|a|*|b|*|c|), Space: O(|a|*|b|*|c|).
public class Solution {
    static int lcs3(String a, String b, String c) {
        int A = a.length(), B = b.length(), C = c.length();
        int[][][] dp = new int[A + 1][B + 1][C + 1];
        for (int i = 1; i <= A; i++)
            for (int j = 1; j <= B; j++)
                for (int k = 1; k <= C; k++) {
                    if (a.charAt(i-1) == b.charAt(j-1) && b.charAt(j-1) == c.charAt(k-1))
                        dp[i][j][k] = dp[i-1][j-1][k-1] + 1;
                    else
                        dp[i][j][k] = Math.max(dp[i-1][j][k],
                                      Math.max(dp[i][j-1][k], dp[i][j][k-1]));
                }
        return dp[A][B][C];
    }

    public static void main(String[] args) {
        System.out.println(lcs3("epidemiologist", "refrigeration",
                "supercalifragilisticexpialodocious")); // 5
    }
}
