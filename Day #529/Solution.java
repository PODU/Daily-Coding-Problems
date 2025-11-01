// Day 529: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
// to reconstruct one optimal partition. Time O(n^2), space O(n^2).
import java.util.*;

public class Solution {
    static List<String> minPalindromePartition(String s) {
        int n = s.length();
        boolean[][] pal = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--)
            for (int j = i; j < n; j++)
                pal[i][j] = s.charAt(i) == s.charAt(j) && (j - i < 2 || pal[i + 1][j - 1]);

        int[] dp = new int[n + 1];
        int[] prev = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        Arrays.fill(prev, -1);
        dp[0] = 0;
        for (int j = 1; j <= n; j++)
            for (int i = 0; i < j; i++)
                if (pal[i][j - 1] && dp[i] != Integer.MAX_VALUE && dp[i] + 1 < dp[j]) {
                    dp[j] = dp[i] + 1;
                    prev[j] = i;
                }

        LinkedList<String> parts = new LinkedList<>();
        for (int j = n; j > 0; j = prev[j]) parts.addFirst(s.substring(prev[j], j));
        return parts;
    }

    public static void main(String[] args) {
        String s = "racecarannakayak";
        List<String> parts = minPalindromePartition(s);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < parts.size(); i++) {
            sb.append("\"").append(parts.get(i)).append("\"");
            if (i + 1 < parts.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb); // expected: ["racecar", "anna", "kayak"]
    }
}
