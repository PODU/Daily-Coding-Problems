// Shortest palindrome by inserting chars; lexicographically earliest on ties.
// DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
public class Solution {
    static String s;
    static int[][] dp;
    static String[][] memo;
    static boolean[][] done;

    static String build(int i, int j) {
        if (i > j) return "";
        if (i == j) return String.valueOf(s.charAt(i));
        if (done[i][j]) return memo[i][j];
        String res;
        if (s.charAt(i) == s.charAt(j)) {
            res = s.charAt(i) + build(i + 1, j - 1) + s.charAt(i);
        } else {
            String a = s.charAt(i) + build(i + 1, j) + s.charAt(i);
            String b = s.charAt(j) + build(i, j - 1) + s.charAt(j);
            if (dp[i + 1][j] < dp[i][j - 1]) res = a;
            else if (dp[i][j - 1] < dp[i + 1][j]) res = b;
            else res = a.compareTo(b) <= 0 ? a : b;
        }
        done[i][j] = true;
        memo[i][j] = res;
        return res;
    }

    static String solve(String str) {
        s = str;
        int n = s.length();
        if (n == 0) return "";
        dp = new int[n][n];
        memo = new String[n][n];
        done = new boolean[n][n];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j)) dp[i][j] = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
                else dp[i][j] = 1 + Math.min(dp[i + 1][j], dp[i][j - 1]);
            }
        }
        return build(0, n - 1);
    }

    public static void main(String[] args) {
        System.out.println("race -> " + solve("race"));
        System.out.println("google -> " + solve("google"));
    }
}
