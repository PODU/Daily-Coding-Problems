// Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
// Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
// Time O(n*k), Space O(n*k).
import java.util.*;

public class Solution {
    static List<Integer> subsetSum(int[] S, int k) {
        int n = S.length;
        boolean[][] dp = new boolean[n + 1][k + 1];
        for (int i = 0; i <= n; i++) dp[i][0] = true;
        for (int i = 1; i <= n; i++)
            for (int j = 1; j <= k; j++) {
                dp[i][j] = dp[i - 1][j];
                if (j >= S[i - 1] && dp[i - 1][j - S[i - 1]]) dp[i][j] = true;
            }
        if (!dp[n][k]) return null;
        List<Integer> res = new ArrayList<>();
        int j = k;
        for (int i = n; i >= 1; i--) {
            if (dp[i - 1][j]) continue;        // item i-1 not needed
            res.add(S[i - 1]);                  // item i-1 must be included
            j -= S[i - 1];
        }
        res.sort(Collections.reverseOrder());
        return res;
    }

    static String fmt(List<Integer> v) {
        if (v == null) return "null";
        StringBuilder s = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) {
            s.append(v.get(i));
            if (i + 1 < v.size()) s.append(", ");
        }
        return s.append("]").toString();
    }

    public static void main(String[] args) {
        int[] S = {12, 1, 61, 5, 9, 2};
        System.out.println(fmt(subsetSum(S, 24)));
    }
}
