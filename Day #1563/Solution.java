// Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).
public class Solution {
    static long countPaths(int n, int m) {
        int total = n + m - 2;
        int k = Math.min(n - 1, m - 1);
        long res = 1;
        for (int i = 1; i <= k; i++) {
            res = res * (total - k + i) / i;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(countPaths(2, 2));
    }
}
