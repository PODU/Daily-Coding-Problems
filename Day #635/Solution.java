// Day 635: Regular expression matching with '.' and '*' (full-string match).
// Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
// Time: O(m*n), Space: O(m*n).
public class Solution {
    static boolean isMatch(String s, String p) {
        int m = s.length(), n = p.length();
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[m][n] = true;
        for (int i = m; i >= 0; i--) {
            for (int j = n - 1; j >= 0; j--) {
                boolean first = i < m && (p.charAt(j) == s.charAt(i) || p.charAt(j) == '.');
                if (j + 1 < n && p.charAt(j + 1) == '*') {
                    dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
                } else {
                    dp[i][j] = first && dp[i + 1][j + 1];
                }
            }
        }
        return dp[0][0];
    }

    public static void main(String[] args) {
        System.out.println(isMatch("ray", "ra."));     // true
        System.out.println(isMatch("raymond", "ra.")); // false
        System.out.println(isMatch("chat", ".*at"));   // true
        System.out.println(isMatch("chats", ".*at"));  // false
    }
}
