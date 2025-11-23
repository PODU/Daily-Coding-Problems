// Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
// Time O(n*sum), Space O(sum).
public class Solution {
    static boolean canPartition(int[] nums) {
        int total = 0;
        for (int x : nums) total += x;
        if (total % 2 != 0) return false;
        int target = total / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;
        for (int x : nums)
            for (int j = target; j >= x; --j)
                if (dp[j - x]) dp[j] = true;
        return dp[target];
    }

    public static void main(String[] args) {
        int[] a = {15, 5, 20, 10, 35, 15, 10};
        int[] b = {15, 5, 20, 10, 35};
        System.out.println(canPartition(a));
        System.out.println(canPartition(b));
    }
}
