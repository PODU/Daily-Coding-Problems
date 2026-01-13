// Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
// Time: O(min(N,M)), Space: O(1).
public class Solution {
    static long paths(long n, long m) {
        long total = n + m - 2;
        long k = Math.min(n - 1, m - 1);
        long res = 1;
        for (long i = 1; i <= k; i++) {
            res = res * (total - k + i) / i;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(paths(2, 2));
        System.out.println(paths(5, 5));
    }
}
