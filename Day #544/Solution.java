// Subset-sum DP with reconstruction. O(n*k) time, O(n*k) space. Result sorted desc.
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
        res.sort(Collections.reverseOrder());
        return res;
    }

    static void printRes(List<Integer> r) {
        if (r == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(r.get(i));
        }
        sb.append("]");
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        int[] s = {12, 1, 61, 5, 9, 2};
        printRes(subsetSum(s, 24));
        printRes(subsetSum(s, 1000));
    }
}
