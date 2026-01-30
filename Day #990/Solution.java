// Day 990: Count ordered ways to climb N steps using step sizes from set X.
// Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.
public class Solution {
    static long climbWays(int n, int[] X) {
        long[] ways = new long[n + 1];
        ways[0] = 1;
        for (int i = 1; i <= n; i++)
            for (int x : X)
                if (i - x >= 0) ways[i] += ways[i - x];
        return ways[n];
    }

    public static void main(String[] args) {
        System.out.println("N=4, X={1,2}: " + climbWays(4, new int[]{1, 2}));      // expected 5
        System.out.println("N=4, X={1,3,5}: " + climbWays(4, new int[]{1, 3, 5})); // generalized
    }
}
