// Connect 4 engine with O(1)-per-move win detection (scan 4 directions from last disc).
// Time per move O(1), Space O(R*C).
import java.util.*;

public class Solution {
    static final int R = 6, C = 7;
    char[][] g = new char[R][C];
    char cur = 'R'; boolean over = false; char winner = '.';
    Solution() { for (char[] row : g) Arrays.fill(row, '.'); }

    int drop(int col) {
        if (col < 0 || col >= C || over) return -1;
        for (int r = R - 1; r >= 0; r--) if (g[r][col] == '.') { g[r][col] = cur; return r; }
        return -1;
    }
    boolean won(int r, int c) {
        char p = g[r][c];
        int[][] dirs = {{0,1},{1,0},{1,1},{1,-1}};
        for (int[] d : dirs) {
            int cnt = 1;
            for (int s = -1; s <= 1; s += 2) {
                int nr = r + d[0]*s, nc = c + d[1]*s;
                while (nr>=0&&nr<R&&nc>=0&&nc<C&&g[nr][nc]==p) { cnt++; nr+=d[0]*s; nc+=d[1]*s; }
            }
            if (cnt >= 4) return true;
        }
        return false;
    }
    boolean full() { for (int c = 0; c < C; c++) if (g[0][c] == '.') return false; return true; }
    boolean play(int col) {
        int r = drop(col); if (r < 0) return false;
        if (won(r, col)) { over = true; winner = cur; }
        else if (full()) over = true;
        else cur = (cur == 'R' ? 'B' : 'R');
        return true;
    }
    void show() {
        StringBuilder sb = new StringBuilder();
        for (char[] row : g) { for (int i = 0; i < C; i++) { sb.append(row[i]); if (i+1<C) sb.append(' '); } sb.append('\n'); }
        System.out.print(sb);
    }

    public static void main(String[] a) {
        Solution game = new Solution();
        int[] moves = {0,1,0,1,0,1,0}; // R wins vertically in column 0
        for (int m : moves) game.play(m);
        game.show();
        if (game.winner != '.') System.out.println("Player " + game.winner + " wins!");
        else System.out.println("Draw");
    }
}
