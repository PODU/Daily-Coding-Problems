// Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
// always filling the next empty cell. Time exponential worst-case but fast in practice.
public class Solution {
    static int[] rows = new int[9], cols = new int[9], boxes = new int[9];
    static int[][] grid = new int[9][9];

    static int boxIdx(int r, int c) { return (r / 3) * 3 + c / 3; }

    static boolean solve(int pos) {
        if (pos == 81) return true;
        int r = pos / 9, c = pos % 9;
        if (grid[r][c] != 0) return solve(pos + 1);
        int b = boxIdx(r, c);
        for (int d = 1; d <= 9; d++) {
            int bit = 1 << d;
            if (((rows[r] | cols[c] | boxes[b]) & bit) != 0) continue;
            grid[r][c] = d; rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
            if (solve(pos + 1)) return true;
            grid[r][c] = 0; rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
        }
        return false;
    }

    public static void main(String[] args) {
        String[] puzzle = {
            "530070000", "600195000", "098000060",
            "800060003", "400803001", "700020006",
            "060000280", "000419005", "000080079"};
        for (int r = 0; r < 9; r++)
            for (int c = 0; c < 9; c++) {
                grid[r][c] = puzzle[r].charAt(c) - '0';
                if (grid[r][c] != 0) {
                    int bit = 1 << grid[r][c];
                    rows[r] |= bit; cols[c] |= bit; boxes[boxIdx(r, c)] |= bit;
                }
            }
        solve(0);
        StringBuilder sb = new StringBuilder();
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) sb.append(grid[r][c]);
            sb.append("\n");
        }
        System.out.print(sb);
    }
}
