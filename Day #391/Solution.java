// Day 391: Longest common contiguous subsequence (substring) of two histories.
// DP on suffix-run lengths. Time O(n*m), Space O(n*m).
import java.util.*;

public class Solution {
    static List<String> longestCommon(String[] a, String[] b) {
        int n = a.length, m = b.length;
        int[][] dp = new int[n + 1][m + 1];
        int best = 0, endI = 0;
        for (int i = 1; i <= n; i++)
            for (int j = 1; j <= m; j++)
                if (a[i - 1].equals(b[j - 1])) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    if (dp[i][j] > best) { best = dp[i][j]; endI = i; }
                }
        List<String> res = new ArrayList<>();
        for (int i = endI - best; i < endI; i++) res.add(a[i]);
        return res;
    }

    public static void main(String[] args) {
        String[] u1 = {"/home", "/register", "/login", "/user", "/one", "/two"};
        String[] u2 = {"/home", "/red", "/login", "/user", "/one", "/pink"};
        List<String> res = longestCommon(u1, u2);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++)
            sb.append("'").append(res.get(i)).append("'").append(i + 1 < res.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb);
    }
}
