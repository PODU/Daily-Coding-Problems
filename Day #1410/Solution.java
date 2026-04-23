// Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
// check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.
public class Solution {
    static final int COLS = 7, ROWS = 6;
    char[][] grid = new char[ROWS][COLS];
    char turn = 'R';

    Solution() {
        for (char[] row : grid) java.util.Arrays.fill(row, '.');
    }

    char drop(int col) {
        if (col < 0 || col >= COLS || grid[ROWS - 1][col] != '.') return 'X';
        int r = 0;
        while (grid[r][col] != '.') r++;
        grid[r][col] = turn;
        char who = turn;
        turn = (turn == 'R') ? 'B' : 'R';
        return wins(r, col, who) ? who : ' ';
    }

    boolean wins(int r, int c, char p) {
        int[][] dirs = {{0, 1}, {1, 0}, {1, 1}, {1, -1}};
        for (int[] d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2)
                for (int k = 1; k < 4; k++) {
                    int nr = r + d[0] * k * s, nc = c + d[1] * k * s;
                    if (nr < 0 || nr >= ROWS || nc < 0 || nc >= COLS || grid[nr][nc] != p) break;
                    cnt++;
                }
            if (cnt >= 4) return true;
        }
        return false;
    }

    void print() {
        for (int r = ROWS - 1; r >= 0; r--) {
            StringBuilder sb = new StringBuilder();
            for (int c = 0; c < COLS; c++) sb.append(grid[r][c]).append(' ');
            System.out.println(sb.toString().trim());
        }
    }

    public static void main(String[] args) {
        Solution game = new Solution();
        int[] moves = {0, 1, 0, 1, 0, 1, 0}; // Red wins vertically in column 0
        char winner = ' ';
        for (int m : moves) {
            char res = game.drop(m);
            if (res == 'R' || res == 'B') { winner = res; break; }
        }
        game.print();
        if (winner != ' ')
            System.out.println((winner == 'R' ? "Red" : "Black") + " wins!");
        else
            System.out.println("No winner yet.");
    }
}
