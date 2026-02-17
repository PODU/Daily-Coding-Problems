// Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
// Worst-case exponential, fast in practice; Space O(1).
public class Solution {
    static int[] rowM = new int[9], colM = new int[9], boxM = new int[9];
    static int[][] grid = new int[9][9];

    static int boxId(int r, int c) { return (r / 3) * 3 + c / 3; }

    static boolean solve(int pos) {
        if (pos == 81) return true;
        int r = pos / 9, c = pos % 9;
        if (grid[r][c] != 0) return solve(pos + 1);
        int b = boxId(r, c);
        for (int d = 1; d <= 9; d++) {
            int bit = 1 << d;
            if (((rowM[r] | colM[c] | boxM[b]) & bit) == 0) {
                grid[r][c] = d; rowM[r] |= bit; colM[c] |= bit; boxM[b] |= bit;
                if (solve(pos + 1)) return true;
                grid[r][c] = 0; rowM[r] &= ~bit; colM[c] &= ~bit; boxM[b] &= ~bit;
            }
        }
        return false;
    }

    public static void main(String[] a) {
        int[][] puzzle = {
            {5,3,0,0,7,0,0,0,0},
            {6,0,0,1,9,5,0,0,0},
            {0,9,8,0,0,0,0,6,0},
            {8,0,0,0,6,0,0,0,3},
            {4,0,0,8,0,3,0,0,1},
            {7,0,0,0,2,0,0,0,6},
            {0,6,0,0,0,0,2,8,0},
            {0,0,0,4,1,9,0,0,5},
            {0,0,0,0,8,0,0,7,9}};
        for (int r = 0; r < 9; r++)
            for (int c = 0; c < 9; c++) {
                grid[r][c] = puzzle[r][c];
                if (puzzle[r][c] != 0) {
                    int bit = 1 << puzzle[r][c];
                    rowM[r] |= bit; colM[c] |= bit; boxM[boxId(r, c)] |= bit;
                }
            }
        solve(0);
        StringBuilder sb = new StringBuilder();
        for (int r = 0; r < 9; r++) { for (int c = 0; c < 9; c++) sb.append(grid[r][c]); sb.append('\n'); }
        System.out.print(sb);
    }
}
