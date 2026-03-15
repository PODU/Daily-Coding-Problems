// Day 1209: Largest divisible subset.
// Sort, dp[i]=longest chain ending at i with parent pointers. Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static List<Integer> largestDivisibleSubset(int[] a) {
        Arrays.sort(a);
        int n = a.length;
        List<Integer> res = new ArrayList<>();
        if (n == 0) return res;
        int[] dp = new int[n], par = new int[n];
        Arrays.fill(dp, 1); Arrays.fill(par, -1);
        int best = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++)
                if (a[i] % a[j] == 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; par[i] = j; }
            if (dp[i] > dp[best]) best = i;
        }
        for (int i = best; i != -1; i = par[i]) res.add(a[i]);
        Collections.reverse(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(largestDivisibleSubset(new int[]{3, 5, 10, 20, 21})); // [5, 10, 20]
    }
}
