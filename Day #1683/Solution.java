// Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
// Build row/column strings, substring search. Time O(N*M), Space O(N+M).
public class Solution {
    static boolean findWord(String[] grid, String word) {
        int rows = grid.length, cols = grid[0].length();
        for (String row : grid)
            if (row.contains(word)) return true;
        for (int c = 0; c < cols; c++) {
            StringBuilder col = new StringBuilder();
            for (int r = 0; r < rows; r++) col.append(grid[r].charAt(c));
            if (col.toString().contains(word)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        String[] grid = {"FACI", "OBQP", "ANOB", "MASS"};
        System.out.println("'FOAM' -> " + (findWord(grid, "FOAM") ? "true" : "false")); // true
        System.out.println("'MASS' -> " + (findWord(grid, "MASS") ? "true" : "false")); // true
    }
}
