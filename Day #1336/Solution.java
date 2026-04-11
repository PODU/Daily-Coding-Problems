// Day 1336: Count distinct N-Queens arrangements.
// Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).
public class Solution {
    static int count(int n, int row, int cols, int d1, int d2) {
        if (row == n) return 1;
        int total = 0;
        int avail = ((1 << n) - 1) & ~(cols | d1 | d2);
        while (avail != 0) {
            int bit = avail & (-avail);
            avail -= bit;
            total += count(n, row + 1, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1);
        }
        return total;
    }

    static int totalNQueens(int n) { return count(n, 0, 0, 0, 0); }

    public static void main(String[] args) {
        System.out.println("N=1 -> " + totalNQueens(1));
        System.out.println("N=4 -> " + totalNQueens(4));
        System.out.println("N=8 -> " + totalNQueens(8));
    }
}
