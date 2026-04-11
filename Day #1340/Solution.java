// Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
// Overflow-safe multiplicative loop. Time O(N+M), Space O(1).
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
        System.out.println("2 by 2 -> " + countPaths(2, 2));
        System.out.println("5 by 5 -> " + countPaths(5, 5));
    }
}
