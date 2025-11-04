// Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
// O(N*M) time, O(1) extra space.
public class Solution {
    static int minDeletions(String[] grid) {
        if (grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length(), count = 0;
        for (int c = 0; c < cols; c++) {
            for (int r = 1; r < rows; r++) {
                if (grid[r].charAt(c) < grid[r - 1].charAt(c)) { count++; break; }
            }
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(minDeletions(new String[]{"cba", "daf", "ghi"})); // 1
        System.out.println(minDeletions(new String[]{"abcdef"}));            // 0
        System.out.println(minDeletions(new String[]{"zyx", "wvu", "tsr"})); // 3
    }
}
