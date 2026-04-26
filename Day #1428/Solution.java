// Day 1428: Partition array into two subsets minimizing sum difference.
// Approach: subset-sum DP over half the total; reconstruct one subset.
// Time: O(n * sum), Space: O(n * sum).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    public static void main(String[] args) {
        int[] a = {5, 10, 15, 20, 25};
        int n = a.length;
        int total = 0;
        for (int x : a) total += x;
        int half = total / 2;

        boolean[][] dp = new boolean[n + 1][half + 1];
        for (int i = 0; i <= n; i++) dp[i][0] = true;
        for (int i = 1; i <= n; i++)
            for (int s = 0; s <= half; s++) {
                dp[i][s] = dp[i - 1][s];
                if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = true;
            }

        int best = 0;
        for (int s = half; s >= 0; s--)
            if (dp[n][s]) { best = s; break; }

        List<Integer> subset = new ArrayList<>();
        int s = best;
        for (int i = n; i >= 1; i--) {
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
                subset.add(a[i - 1]);
                s -= a[i - 1];
            }
        }

        int diff = total - 2 * best;
        System.out.println("Difference: " + diff);   // Difference: 5
        System.out.println("Subset: " + subset);
    }
}
