// Count ways to roll N dice (faces each) summing to total via DP.
// Time O(N*faces*total), Space O(total).
public class Solution {
    static long throwDice(int N, int faces, int total) {
        long[] dp = new long[total + 1];
        dp[0] = 1;
        for (int d = 0; d < N; d++) {
            long[] nxt = new long[total + 1];
            for (int s = 0; s <= total; s++)
                if (dp[s] != 0)
                    for (int f = 1; f <= faces && s + f <= total; f++)
                        nxt[s + f] += dp[s];
            dp = nxt;
        }
        return dp[total];
    }

    public static void main(String[] args) {
        System.out.println(throwDice(3, 6, 7));
    }
}
