// Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
// LPS via DP. Time O(n^2), Space O(n^2).
public class Solution {
    static int lps(String s) {
        int n = s.length();
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = 1;
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j)) dp[i][j] = (len == 2 ? 2 : dp[i + 1][j - 1] + 2);
                else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
            }
        return dp[0][n - 1];
    }

    static boolean canMakePalindrome(String s, int k) {
        return s.length() - lps(s) <= k;
    }

    public static void main(String[] args) {
        System.out.println(canMakePalindrome("waterrfetawx", 2) ? "True" : "False");
    }
}
