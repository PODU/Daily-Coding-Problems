// Day 1580: Largest divisible subset (every pair mutually divisible).
// Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
// Time: O(n^2); Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> largestDivisible(int[] a) {
        Arrays.sort(a);
        int n = a.length;
        if (n == 0) return new ArrayList<>();
        int[] dp = new int[n], prev = new int[n];
        Arrays.fill(dp, 1);
        Arrays.fill(prev, -1);
        int best = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; prev[i] = j; }
            }
            if (dp[i] > dp[best]) best = i;
        }
        LinkedList<Integer> res = new LinkedList<>();
        for (int i = best; i != -1; i = prev[i]) res.addFirst(a[i]);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(largestDivisible(new int[]{3, 5, 10, 20, 21})); // [5, 10, 20]
    }
}
