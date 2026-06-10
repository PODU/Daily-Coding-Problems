// Day 1636: Can a string be made a palindrome by deleting at most k chars.
// min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.
public class Solution {
    static boolean canMakePalindrome(String s, int k) {
        int n = s.length();
        if (n == 0) return true;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = 1;
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j))
                    dp[i][j] = 2 + (len > 2 ? dp[i + 1][j - 1] : 0);
                else
                    dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
            }
        int lps = dp[0][n - 1];
        return (n - lps) <= k;
    }

    public static void main(String[] args) {
        String s = "waterrfetawx";
        int k = 2;
        System.out.println(canMakePalindrome(s, k) ? "True" : "False");
    }
}
