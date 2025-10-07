// Min coins via bottom-up DP. Returns null (Integer) if unreachable.
// Time: O(amount * |coins|), Space: O(amount).
public class Solution {
    static Integer minCoins(int[] coins, int amount){
        int INF = Integer.MAX_VALUE;
        int[] dp = new int[amount+1];
        java.util.Arrays.fill(dp, INF);
        dp[0] = 0;
        for(int a=1;a<=amount;a++)
            for(int c : coins)
                if(c <= a && dp[a-c] != INF) dp[a] = Math.min(dp[a], dp[a-c]+1);
        return dp[amount] == INF ? null : dp[amount];
    }

    static void show(int[] coins, int amount){
        Integer r = minCoins(coins, amount);
        System.out.println(r == null ? "null" : r.toString());
    }

    public static void main(String[] args){
        show(new int[]{1,5,10}, 56);
        show(new int[]{5,8}, 15);
    }
}
