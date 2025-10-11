// Day 411: Can we make s a palindrome by deleting at most k chars?
// Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
public class Solution {
    static int lps(String s) {
        int n = s.length();
        int[][] dp = new int[n][n];
        for (int i = n - 1; i >= 0; i--) {
            dp[i][i] = 1;
            for (int j = i + 1; j < n; j++) {
                if (s.charAt(i) == s.charAt(j)) dp[i][j] = dp[i + 1][j - 1] + 2;
                else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
        return dp[0][n - 1];
    }

    static boolean canMakePalindrome(String s, int k) {
        return s.length() - lps(s) <= k;
    }

    public static void main(String[] args) {
        String s = "waterrfetawx";
        int k = 2;
        System.out.println(canMakePalindrome(s, k) ? "True" : "False");
    }
}
