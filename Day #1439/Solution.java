// Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
// Approach: build each row and column string, check if target is a substring.
// Time: O(R*C*L) substring scan, Space: O(R+C).
public class Solution {
    static boolean findWord(char[][] grid, String target) {
        int rows = grid.length;
        if (rows == 0) return false;
        int cols = grid[0].length;
        for (char[] row : grid) {
            if (new String(row).contains(target)) return true;
        }
        for (int c = 0; c < cols; c++) {
            StringBuilder col = new StringBuilder();
            for (int r = 0; r < rows; r++) col.append(grid[r][c]);
            if (col.toString().contains(target)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        char[][] grid = {
            {'F', 'A', 'C', 'I'},
            {'O', 'B', 'Q', 'P'},
            {'A', 'N', 'O', 'B'},
            {'M', 'A', 'S', 'S'}
        };
        System.out.println(findWord(grid, "FOAM")); // true
        System.out.println(findWord(grid, "MASS")); // true
    }
}
