// Day 1328: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.
import java.util.*;

public class Solution {
    static List<String> minPalindromePartition(String s) {
        int n = s.length();
        List<String> res = new ArrayList<>();
        if (n == 0) return res;
        boolean[][] pal = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--)
            for (int j = i; j < n; j++)
                pal[i][j] = s.charAt(i) == s.charAt(j) && (j - i < 2 || pal[i + 1][j - 1]);

        int[] dp = new int[n + 1], prev = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int i = 1; i <= n; i++)
            for (int j = 0; j < i; j++)
                if (pal[j][i - 1] && dp[j] != Integer.MAX_VALUE && dp[j] + 1 < dp[i]) {
                    dp[i] = dp[j] + 1; prev[i] = j;
                }

        for (int i = n; i > 0; i = prev[i]) res.add(s.substring(prev[i], i));
        Collections.reverse(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(minPalindromePartition("racecarannakayak")); // [racecar, anna, kayak]
        System.out.println(minPalindromePartition("abc"));               // [a, b, c]
    }
}
