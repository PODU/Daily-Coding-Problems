// Day 733: Connect 4 on a 7x6 grid.
// Approach: 2D board + per-column heights; after each drop scan 4 directions from the
// placed disc for 4-in-a-row. Time: O(1) per move, Space: O(rows*cols).
public class Solution {
    static final int COLS = 7, ROWS = 6;

    static class Connect4 {
        char[][] board = new char[ROWS][COLS];
        int[] height = new int[COLS];
        Connect4() { for (char[] row : board) java.util.Arrays.fill(row, '.'); }

        boolean drop(int col, char color) {
            int r = height[col]++;
            board[r][col] = color;
            int[][] dirs = {{0, 1}, {1, 0}, {1, 1}, {1, -1}};
            for (int[] d : dirs) {
                int cnt = 1;
                for (int s = -1; s <= 1; s += 2)
                    for (int k = 1; k < 4; k++) {
                        int nr = r + d[0] * k * s, nc = col + d[1] * k * s;
                        if (nr < 0 || nr >= ROWS || nc < 0 || nc >= COLS || board[nr][nc] != color) break;
                        cnt++;
                    }
                if (cnt >= 4) return true;
            }
            return false;
        }

        void print() {
            for (int r = ROWS - 1; r >= 0; r--) {
                StringBuilder sb = new StringBuilder();
                for (int c = 0; c < COLS; c++) sb.append(board[r][c]).append(c + 1 < COLS ? " " : "");
                System.out.println(sb);
            }
        }
    }

    public static void main(String[] args) {
        Connect4 game = new Connect4();
        int[] moves = {0, 0, 1, 1, 2, 2, 3};
        char color = 'R';
        boolean won = false;
        for (int m : moves) {
            won = game.drop(m, color);
            if (won) break;
            color = (color == 'R') ? 'B' : 'R';
        }
        game.print();
        System.out.println(won ? (color == 'R' ? "Red wins!" : "Black wins!") : "No winner yet");
    }
}
