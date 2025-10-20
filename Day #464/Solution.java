// Day 464: Largest divisible subset.
// Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
// j divides i means a[i]%a[j]==0. Reconstruct via parent pointers.
// Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> largestDivisibleSubset(int[] nums) {
        List<Integer> res = new ArrayList<>();
        if (nums.length == 0) return res;
        Arrays.sort(nums);
        int n = nums.length;
        int[] dp = new int[n], parent = new int[n];
        Arrays.fill(dp, 1);
        Arrays.fill(parent, -1);
        int best = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i]) {
                    dp[i] = dp[j] + 1;
                    parent[i] = j;
                }
            }
            if (dp[i] > dp[best]) best = i;
        }
        for (int i = best; i >= 0; i = parent[i]) res.add(nums[i]);
        Collections.reverse(res);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(largestDivisibleSubset(new int[]{3, 5, 10, 20, 21}));
        System.out.println(largestDivisibleSubset(new int[]{1, 3, 6, 24}));
    }
}
