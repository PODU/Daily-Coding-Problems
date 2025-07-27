// N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
public class Solution {
    static int solve(int cols, int diag1, int diag2, int full) {
        if (cols == full) return 1;
        int count = 0;
        int avail = ~(cols | diag1 | diag2) & full;
        while (avail != 0) {
            int bit = avail & (-avail);
            avail -= bit;
            count += solve(cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1, full);
        }
        return count;
    }

    static int countNQueens(int n) {
        return solve(0, 0, 0, (1 << n) - 1);
    }

    public static void main(String[] args) {
        System.out.println(countNQueens(8));
    }
}
