// Subset sum returning an actual subset via DP + backtracking reconstruction.
// Time O(n*k), Space O(n*k). Returns null if no subset sums to k.
import java.util.*;

public class Solution {
    static List<Integer> subsetSum(int[] s, int k) {
        int n = s.length;
        boolean[][] dp = new boolean[n + 1][k + 1];
        for (int i = 0; i <= n; i++) dp[i][0] = true;
        for (int i = 1; i <= n; i++)
            for (int j = 0; j <= k; j++) {
                dp[i][j] = dp[i - 1][j];
                if (j >= s[i - 1] && dp[i - 1][j - s[i - 1]]) dp[i][j] = true;
            }
        if (!dp[n][k]) return null;
        List<Integer> res = new ArrayList<>();
        int j = k;
        for (int i = n; i >= 1; i--) {
            if (!dp[i - 1][j]) { res.add(s[i - 1]); j -= s[i - 1]; }
        }
        Collections.reverse(res);
        return res;
    }

    public static void main(String[] args) {
        List<Integer> res = subsetSum(new int[]{12, 1, 61, 5, 9, 2}, 24);
        System.out.println(res == null ? "null" : res);
    }
}
