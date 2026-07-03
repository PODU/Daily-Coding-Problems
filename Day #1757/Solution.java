// Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
// Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.
import java.util.*;

public class Solution {
    static long climbWays(int n, int[] steps) {
        long[] ways = new long[n + 1];
        ways[0] = 1;
        for (int i = 1; i <= n; i++)
            for (int x : steps)
                if (i - x >= 0) ways[i] += ways[i - x];
        return ways[n];
    }

    public static void main(String[] args) {
        int N = 4;
        System.out.println(climbWays(N, new int[]{1, 2})); // 5
        System.out.println("Generalized X={1,3,5}, N=4: " + climbWays(N, new int[]{1, 3, 5}));
    }
}
