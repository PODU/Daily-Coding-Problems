// Day 1684: Levenshtein edit distance via 1D rolling DP.
// Time O(n*m), Space O(min(n,m)).
public class Solution {
    static int editDistance(String a, String b) {
        int n = a.length(), m = b.length();
        int[] dp = new int[m + 1];
        for (int j = 0; j <= m; j++) dp[j] = j;
        for (int i = 1; i <= n; i++) {
            int prev = dp[0];
            dp[0] = i;
            for (int j = 1; j <= m; j++) {
                int tmp = dp[j];
                if (a.charAt(i - 1) == b.charAt(j - 1)) dp[j] = prev;
                else dp[j] = 1 + Math.min(prev, Math.min(dp[j], dp[j - 1]));
                prev = tmp;
            }
        }
        return dp[m];
    }

    public static void main(String[] args) {
        System.out.println(editDistance("kitten", "sitting")); // 3
    }
}
