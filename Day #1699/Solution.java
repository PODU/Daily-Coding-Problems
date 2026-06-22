// Count ways to roll N dice (faces each) summing to total via rolling 1D DP.
// Time O(N*total*faces), Space O(total).
public class Solution {
    static long throwDice(int n, int faces, int total) {
        long[] dp = new long[total + 1];
        dp[0] = 1;
        for (int k = 0; k < n; k++) {
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
