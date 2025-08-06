// Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
// Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).
public class Solution {
    static int minColumnsToRemove(String[] grid) {
        if (grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length();
        int remove = 0;
        for (int c = 0; c < cols; c++) {
            for (int r = 1; r < rows; r++) {
                if (grid[r].charAt(c) < grid[r - 1].charAt(c)) { remove++; break; }
            }
        }
        return remove;
    }

    public static void main(String[] args) {
        String[] grid = {"cba", "daf", "ghi"};
        System.out.println(minColumnsToRemove(grid)); // 1
    }
}
