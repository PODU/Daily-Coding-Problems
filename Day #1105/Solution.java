// Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
// If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
public class Solution {
    static boolean canPartition(int[] nums){
        int total = 0;
        for (int x : nums) total += x;
        if ((total & 1) == 1) return false;
        int target = total / 2;
        boolean[] dp = new boolean[target+1];
        dp[0] = true;
        for (int x : nums)
            for (int s = target; s >= x; s--)
                if (dp[s-x]) dp[s] = true;
        return dp[target];
    }
    public static void main(String[] args){
        System.out.println(canPartition(new int[]{15,5,20,10,35,15,10})); // true
        System.out.println(canPartition(new int[]{15,5,20,10,35}));        // false
    }
}
