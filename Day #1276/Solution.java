// Day 1276: Partition a multiset into two subsets of equal sum.
// Subset-sum DP (can we reach totalSum/2?). Time O(n*S), Space O(S).
public class Solution {
    static boolean canPartition(int[] nums) {
        int total = 0;
        for (int x : nums) total += x;
        if (total % 2 != 0) return false;
        int target = total / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;
        for (int x : nums)
            for (int s = target; s >= x; --s)
                if (dp[s - x]) dp[s] = true;
        return dp[target];
    }

    public static void main(String[] args) {
        System.out.println(canPartition(new int[]{15, 5, 20, 10, 35, 15, 10}));
        System.out.println(canPartition(new int[]{15, 5, 20, 10, 35}));
    }
}
