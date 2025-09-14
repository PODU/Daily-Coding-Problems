// Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
// 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).
public class Solution {
    static long throwDice(int N, int faces, int total) {
        long[] dp = new long[total + 1];
        dp[0] = 1;
        for (int d = 0; d < N; d++) {
            long[] ndp = new long[total + 1];
            for (int t = 0; t <= total; t++) {
                if (dp[t] == 0) continue;
                for (int f = 1; f <= faces && t + f <= total; f++)
                    ndp[t + f] += dp[t];
            }
            dp = ndp;
        }
        return dp[total];
    }

    public static void main(String[] args) {
        System.out.println(throwDice(3, 6, 7)); // 15
    }
}
