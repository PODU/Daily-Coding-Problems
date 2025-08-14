// Day 121: Can form palindrome by deleting at most k chars.
// Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.
public class Solution {
    static int lps(String s) {
        int n = s.length();
        if (n == 0) return 0;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = 1;
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j)) dp[i][j] = 2 + (len == 2 ? 0 : dp[i + 1][j - 1]);
                else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
            }
        return dp[0][n - 1];
    }

    static boolean canMakePalindrome(String s, int k) {
        return s.length() - lps(s) <= k;
    }

    public static void main(String[] args) {
        String s = "waterrfetawx";
        int k = 2;
        System.out.println(canMakePalindrome(s, k)
                ? "You could delete f and x to get 'waterretaw'."
                : "Not possible");
    }
}
