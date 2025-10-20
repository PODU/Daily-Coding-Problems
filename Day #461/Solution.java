// Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
// Multiplicative binomial. Time O(min(N,M)), Space O(1).
public class Solution {
    static long countPaths(int n, int m) {
        int a = (n - 1) + (m - 1), b = Math.min(n - 1, m - 1);
        long res = 1;
        for (int i = 1; i <= b; i++) {
            res = res * (a - b + i) / i;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(countPaths(2, 2)); // 2
        System.out.println(countPaths(5, 5)); // 70
    }
}
