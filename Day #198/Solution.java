// Day 198: Largest divisible subset.
// Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
// Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> largestDivisibleSubset(int[] a) {
        int n = a.length;
        if (n == 0) return new ArrayList<>();
        Arrays.sort(a);
        int[] dp = new int[n], parent = new int[n];
        Arrays.fill(dp, 1);
        Arrays.fill(parent, -1);
        int best = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++)
                if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; parent[i] = j; }
            if (dp[i] > dp[best]) best = i;
        }
        LinkedList<Integer> res = new LinkedList<>();
        for (int i = best; i != -1; i = parent[i]) res.addFirst(a[i]);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(largestDivisibleSubset(new int[]{3, 5, 10, 20, 21})); // [5, 10, 20]
        System.out.println(largestDivisibleSubset(new int[]{1, 3, 6, 24}));      // [1, 3, 6, 24]
    }
}
