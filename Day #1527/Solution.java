// Count cells equal to X in an N x N multiplication table (cell(i,j)=i*j).
// For each row i, X is at column X/i iff i divides X and 1<=X/i<=N. O(N) time, O(1) space.
public class Solution {
    static int countCells(long n, long x) {
        int count = 0;
        for (long i = 1; i <= n; i++) {
            if (x % i == 0) {
                long j = x / i;
                if (j >= 1 && j <= n) count++;
            }
        }
        return count;
    }

    public static void main(String[] args) {
        long n = 6, x = 12;
        System.out.println(countCells(n, x)); // expected 4
    }
}
