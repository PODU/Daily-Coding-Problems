// Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
public class Solution {
    static String solve(String s) {
        int n = s.length();
        if (n == 0) return "";
        String[][] dp = new String[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = String.valueOf(s.charAt(i));
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j)) {
                    String inner = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : "";
                    dp[i][j] = s.charAt(i) + inner + s.charAt(j);
                } else {
                    String c1 = s.charAt(i) + dp[i + 1][j] + s.charAt(i);
                    String c2 = s.charAt(j) + dp[i][j - 1] + s.charAt(j);
                    if (c1.length() < c2.length()) dp[i][j] = c1;
                    else if (c2.length() < c1.length()) dp[i][j] = c2;
                    else dp[i][j] = c1.compareTo(c2) <= 0 ? c1 : c2;
                }
            }
        }
        return dp[0][n - 1];
    }

    public static void main(String[] args) {
        System.out.println("\"" + solve("race") + "\"");
    }
}
