// Day 413: Ordered ways to climb a staircase with allowed step sizes X.
// DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
public class Solution {
    static long countWays(int n, int[] steps) {
        long[] ways = new long[n + 1];
        ways[0] = 1;
        for (int i = 1; i <= n; i++)
            for (int x : steps)
                if (x <= i) ways[i] += ways[i - x];
        return ways[n];
    }

    public static void main(String[] args) {
        System.out.println(countWays(4, new int[]{1, 2}));       // 5
        System.out.println(countWays(10, new int[]{1, 3, 5}));   // generalized X={1,3,5}
    }
}
