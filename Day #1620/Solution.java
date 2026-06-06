// Sudoku solver: backtracking with row/col/box bitmasks; pick first empty cell.
// Time: exponential worst case, fast in practice. Space: O(1) extra (fixed 9x9).
public class Solution {
    static int[] rows = new int[9], cols = new int[9], boxes = new int[9];
    static char[][] grid = new char[9][9];

    static int boxIndex(int r, int c) { return (r / 3) * 3 + c / 3; }

    static boolean solve(int pos) {
        if (pos == 81) return true;
        int r = pos / 9, c = pos % 9;
        if (grid[r][c] != '0' && grid[r][c] != '.')
            return solve(pos + 1);
        int b = boxIndex(r, c);
        for (int d = 1; d <= 9; d++) {
            int bit = 1 << d;
            if ((rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[b] & bit) != 0) continue;
            rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
            grid[r][c] = (char) ('0' + d);
            if (solve(pos + 1)) return true;
            rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
            grid[r][c] = '.';
        }
        return false;
    }

    public static void main(String[] args) {
        String[] puzzle = {
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79"
        };
        for (int r = 0; r < 9; r++)
            for (int c = 0; c < 9; c++) {
                char ch = puzzle[r].charAt(c);
                grid[r][c] = ch;
                if (ch != '.' && ch != '0') {
                    int bit = 1 << (ch - '0');
                    rows[r] |= bit; cols[c] |= bit; boxes[boxIndex(r, c)] |= bit;
                }
            }
        solve(0);
        StringBuilder sb = new StringBuilder();
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) sb.append(grid[r][c]);
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
