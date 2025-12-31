// Day 828: Count distinct max heaps from N distinct integers.
// Root is the max; f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size from
// complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
public class Solution {
    static long[][] C;

    static int leftCount(int m) {
        if (m == 1) return 0;
        int h = 31 - Integer.numberOfLeadingZeros(m);  // height (0-indexed)
        int last = m - ((1 << h) - 1);
        int leftCap = 1 << (h - 1);
        return ((1 << (h - 1)) - 1) + Math.min(leftCap, last);
    }

    static long f(int m) {
        if (m <= 1) return 1;
        int L = leftCount(m);
        int R = m - 1 - L;
        return C[m - 1][L] * f(L) * f(R);
    }

    static long countMaxHeaps(int n) {
        C = new long[n + 1][n + 1];
        for (int i = 0; i <= n; i++) {
            C[i][0] = 1;
            for (int j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
        }
        return f(n);
    }

    public static void main(String[] args) {
        System.out.println(countMaxHeaps(3));
    }
}
