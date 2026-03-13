// Recover coin denominations from change-ways array A (unbounded coin change).
// dp starts {1,0,...}; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.
import java.util.*;

public class Solution {
    static List<Integer> recoverCoins(int[] A) {
        int n = A.length;
        long[] dp = new long[n];
        dp[0] = 1;
        List<Integer> coins = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (A[i] != dp[i]) {
                coins.add(i);
                for (int v = i; v < n; v++) dp[v] += dp[v - i];
            }
        }
        return coins;
    }

    static String formatList(List<Integer> xs) {
        int n = xs.size();
        StringBuilder s = new StringBuilder();
        for (int i = 0; i < n; i++) {
            if (i > 0) s.append(", ");
            if (i == n - 1 && n > 1) s.append("and ");
            s.append(xs.get(i));
        }
        return s.toString();
    }

    public static void main(String[] args) {
        int[] A = {1, 0, 1, 1, 2};
        System.out.println(formatList(recoverCoins(A))); // 2, 3, and 4
    }
}
