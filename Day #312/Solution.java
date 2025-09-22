// Day 312: Tilings of a 2xN board with dominoes & L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3). O(N) time.
public class Solution {
    static long tilings(int n) {
        if (n == 0) return 1;
        if (n == 1) return 1;
        if (n == 2) return 2;
        long[] f = new long[n + 1];
        f[0] = 1; f[1] = 1; f[2] = 2;
        for (int i = 3; i <= n; i++) f[i] = 2 * f[i - 1] + f[i - 3];
        return f[n];
    }
    public static void main(String[] a) {
        System.out.println(tilings(4)); // 11
    }
}
