// Day 527: Count distinct max-heaps from N distinct integers.
// Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
// binary tree with n nodes. BigInteger for exactness. Time O(n^2), space O(n^2).
import java.math.BigInteger;

public class Solution {
    // number of nodes in the left subtree of a complete binary tree with n nodes
    static long leftSubtreeSize(long n) {
        if (n <= 1) return 0;
        int h = 0;
        while ((1L << (h + 1)) - 1 <= n) h++; // h = height (root at height 0)
        long lastLevelCap = 1L << h;
        long nodesAbove = (1L << h) - 1;
        long lastLevelNodes = n - nodesAbove;
        long leftBase = (1L << (h - 1)) - 1;
        long leftLast = Math.min(lastLevelNodes, lastLevelCap / 2);
        return leftBase + leftLast;
    }

    static BigInteger binom(int n, int k) {
        BigInteger r = BigInteger.ONE;
        for (int i = 0; i < k; i++)
            r = r.multiply(BigInteger.valueOf(n - i)).divide(BigInteger.valueOf(i + 1));
        return r;
    }

    static BigInteger countHeaps(long n) {
        if (n <= 1) return BigInteger.ONE;
        long L = leftSubtreeSize(n);
        long R = n - 1 - L;
        return binom((int) (n - 1), (int) L).multiply(countHeaps(L)).multiply(countHeaps(R));
    }

    public static void main(String[] args) {
        int N = 3;
        int[] integers = {1, 2, 3};
        System.out.println(countHeaps(N)); // expected: 2
    }
}
