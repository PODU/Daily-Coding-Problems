// N-Queens count via backtracking with bitmasks (columns + two diagonals).
// Time O(N!) worst case (heavily pruned), Space O(N) recursion.
public class Solution {
    static int solve(int n, int row, int cols, int diag1, int diag2) {
        if (row == n) return 1;
        int count = 0;
        int avail = ((1 << n) - 1) & ~(cols | diag1 | diag2);
        while (avail != 0) {
            int bit = avail & (-avail);
            avail -= bit;
            count += solve(n, row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
        }
        return count;
    }

    static int totalNQueens(int n) {
        return solve(n, 0, 0, 0, 0);
    }

    public static void main(String[] args) {
        System.out.println(totalNQueens(8)); // 92
    }
}
