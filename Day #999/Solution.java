// Day 999: Count occurrences of X in an N x N multiplication table.
// X appears at (i, j) iff i divides X and X/i <= N, for i in 1..N. O(N) time.
public class Solution {
    static int countX(int n, int x) {
        int cnt = 0;
        for (int i = 1; i <= n; i++)
            if (x % i == 0 && x / i <= n) cnt++;
        return cnt;
    }

    public static void main(String[] args) {
        System.out.println(countX(6, 12)); // 4
    }
}
