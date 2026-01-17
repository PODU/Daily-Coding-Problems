// Count columns to delete so each remaining column is non-decreasing top->bottom.
// Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).
public class Solution {
    static int minDeletions(String[] grid) {
        if (grid.length == 0) return 0;
        int n = grid.length, m = grid[0].length(), count = 0;
        for (int c = 0; c < m; c++)
            for (int i = 0; i + 1 < n; i++)
                if (grid[i].charAt(c) > grid[i + 1].charAt(c)) { count++; break; }
        return count;
    }

    public static void main(String[] args) {
        String[] grid = {"cba", "daf", "ghi"};
        System.out.println(minDeletions(grid));
    }
}
