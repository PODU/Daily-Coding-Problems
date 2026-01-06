// Day 860: Regex matching with '.' and '*'.
// Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
// Time: O(n*m), Space: O(n*m).
public class Solution {
    static boolean isMatch(String s, String p) {
        int n = s.length(), m = p.length();
        boolean[][] dp = new boolean[n + 1][m + 1];
        dp[n][m] = true;
        for (int i = n; i >= 0; i--)
            for (int j = m - 1; j >= 0; j--) {
                boolean first = i < n && (p.charAt(j) == s.charAt(i) || p.charAt(j) == '.');
                if (j + 1 < m && p.charAt(j + 1) == '*')
                    dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
                else
                    dp[i][j] = first && dp[i + 1][j + 1];
            }
        return dp[0][0];
    }

    public static void main(String[] args) {
        System.out.println(isMatch("ray", "ra."));      // true
        System.out.println(isMatch("raymond", "ra."));  // false
        System.out.println(isMatch("chat", ".*at"));    // true
        System.out.println(isMatch("chats", ".*at"));   // false
    }
}
