// Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
// drop() places a disc; win() scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).
public class Solution {
    static final int COLS = 7, ROWS = 6;
    char[][] g = new char[ROWS][COLS];

    Solution() { for (char[] row : g) java.util.Arrays.fill(row, '.'); }

    int drop(int col, char p) {
        for (int r = ROWS - 1; r >= 0; r--) if (g[r][col] == '.') { g[r][col] = p; return r; }
        return -1;
    }

    boolean win(int r, int c) {
        char p = g[r][c];
        int[][] dirs = {{0,1},{1,0},{1,1},{1,-1}};
        for (int[] d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2) {
                int rr = r + s*d[0], cc = c + s*d[1];
                while (rr>=0&&rr<ROWS&&cc>=0&&cc<COLS&&g[rr][cc]==p) { cnt++; rr+=s*d[0]; cc+=s*d[1]; }
            }
            if (cnt >= 4) return true;
        }
        return false;
    }

    void print() {
        for (char[] row : g) { StringBuilder sb = new StringBuilder();
            for (char ch : row) sb.append(ch).append(' '); System.out.println(sb.toString().trim()); }
    }

    public static void main(String[] args) {
        Solution game = new Solution();
        int[][] moves = {{0,'R'},{1,'B'},{0,'R'},{1,'B'},{0,'R'},{1,'B'},{0,'R'}};
        char winner = 0;
        for (int[] m : moves) {
            int r = game.drop(m[0], (char) m[1]);
            if (game.win(r, m[0])) { winner = (char) m[1]; break; }
        }
        game.print();
        System.out.println("Winner: " + (winner != 0 ? String.valueOf(winner) : "none")); // R
    }
}
