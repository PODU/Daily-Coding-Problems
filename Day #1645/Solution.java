// Regex full-match with '.' and '*' via DP: dp[i][j] = s[0..i) matches p[0..j).
// '*' uses zero-copy (dp[i][j-2]) or one-more (prev char match). Time/Space O(m*n).
public class Solution {
    static boolean isMatch(String s, String p) {
        int m = s.length(), n = p.length();
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;
        for (int j = 1; j <= n; j++)
            if (p.charAt(j - 1) == '*' && j >= 2) dp[0][j] = dp[0][j - 2];
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                char pc = p.charAt(j - 1);
                if (pc == '*') {
                    dp[i][j] = (j >= 2 && dp[i][j - 2]);
                    char prev = j >= 2 ? p.charAt(j - 2) : 0;
                    if (j >= 2 && (prev == '.' || prev == s.charAt(i - 1)))
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                } else if (pc == '.' || pc == s.charAt(i - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        return dp[m][n];
    }

    public static void main(String[] args) {
        System.out.println(isMatch("ray", "ra."));
        System.out.println(isMatch("raymond", "ra."));
        System.out.println(isMatch("chat", ".*at"));
        System.out.println(isMatch("chats", ".*at"));
    }
}
