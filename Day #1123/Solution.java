// Day 1123 - Largest divisible subset
// Sort, LIS-style DP where j extends i if a[i] % a[j] == 0; reconstruct via
// parent pointers. Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> largestDivisibleSubset(int[] nums) {
        if (nums.length == 0) return new ArrayList<>();
        Arrays.sort(nums);
        int n = nums.length;
        int[] dp = new int[n], parent = new int[n];
        Arrays.fill(dp, 1);
        Arrays.fill(parent, -1);
        int best = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++)
                if (nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i]) {
                    dp[i] = dp[j] + 1;
                    parent[i] = j;
                }
            if (dp[i] > dp[best]) best = i;
        }
        LinkedList<Integer> res = new LinkedList<>();
        for (int k = best; k != -1; k = parent[k]) res.addFirst(nums[k]);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(largestDivisibleSubset(new int[]{3, 5, 10, 20, 21})); // [5, 10, 20]
        System.out.println(largestDivisibleSubset(new int[]{1, 3, 6, 24}));      // [1, 3, 6, 24]
    }
}
