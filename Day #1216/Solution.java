// Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
// Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
public class Solution {
    static int minDeletions(String[] grid) {
        if (grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length(), count = 0;
        for (int c = 0; c < cols; c++)
            for (int r = 1; r < rows; r++)
                if (grid[r].charAt(c) < grid[r-1].charAt(c)) { count++; break; }
        return count;
    }

    public static void main(String[] args) {
        String[] grid = {"cba", "daf", "ghi"};
        System.out.println(minDeletions(grid)); // 1
    }
}
