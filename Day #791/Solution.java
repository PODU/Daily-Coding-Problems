// throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
// Time O(N*total*faces), Space O(total).
public class Solution {
    static long throwDice(int N, int faces, int total) {
        long[] dp = new long[total + 1];
        dp[0] = 1;
        for (int i = 0; i < N; i++) {
            long[] ndp = new long[total + 1];
            for (int s = 0; s <= total; s++) {
                if (dp[s] == 0) continue;
                for (int f = 1; f <= faces && s + f <= total; f++)
                    ndp[s + f] += dp[s];
            }
            dp = ndp;
        }
        return dp[total];
    }

    public static void main(String[] args) {
        System.out.println(throwDice(3, 6, 7));
    }
}
