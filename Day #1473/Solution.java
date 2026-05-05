// Day 1473: Count occurrences of X in an N x N multiplication table.
// For each row i, X appears iff i divides X and X/i is within [1, N].
// Time O(N), Space O(1).
public class Solution {
    static long countX(long n, long x) {
        long count = 0;
        for (long i = 1; i <= n; ++i) {
            if (x % i == 0 && x / i >= 1 && x / i <= n) ++count;
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(countX(6, 12));  // 4
    }
}
