// Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Solution {
    static List<Integer> subsetSum(int[] items, int k) {
        int n = items.length;
        boolean[][] dp = new boolean[n+1][k+1];
        dp[0][0] = true;
        for (int i = 1; i <= n; i++) {
            for (int s = 0; s <= k; s++) {
                dp[i][s] = dp[i-1][s];
                if (!dp[i][s] && s >= items[i-1])
                    dp[i][s] = dp[i-1][s - items[i-1]];
            }
        }
        if (!dp[n][k]) return null;
        List<Integer> result = new ArrayList<>();
        int s = k;
        for (int i = n; i >= 1; i--) {
            if (!dp[i-1][s]) {
                result.add(items[i-1]);
                s -= items[i-1];
            }
        }
        Collections.reverse(result);
        return result;
    }

    public static void main(String[] args) {
        int[] items = {12, 1, 61, 5, 9, 2};
        List<Integer> res = subsetSum(items, 24);
        System.out.println("Subset: " + res);
        int sum = res.stream().mapToInt(Integer::intValue).sum();
        System.out.println("Sum: " + sum);

        List<Integer> res2 = subsetSum(items, 1000);
        System.out.println(res2 == null ? "null" : res2.toString());
    }
}
