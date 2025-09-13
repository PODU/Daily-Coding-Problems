// Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
// rays and bishop/queen diagonal rays (stopping at the first blocker), and test
// knight and pawn attacks. Time O(8x8) = O(1); space O(1).
public class Solution {
    static char[][] B;

    static char at(int r, int c) {
        if (r < 0 || r >= 8 || c < 0 || c >= 8) return 0;
        return B[r][c];
    }

    static boolean inCheck(String[] board) {
        B = new char[8][];
        for (int i = 0; i < 8; i++) B[i] = board[i].toCharArray();
        int kr = -1, kc = -1;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (B[r][c] == 'K') { kr = r; kc = c; }
        if (kr < 0) return false;

        int[][] orth = {{1,0},{-1,0},{0,1},{0,-1}};
        for (int[] d : orth) {
            int r = kr + d[0], c = kc + d[1];
            while (at(r, c) == '.') { r += d[0]; c += d[1]; }
            char p = at(r, c);
            if (p == 'R' || p == 'Q') return true;
        }
        int[][] diag = {{1,1},{1,-1},{-1,1},{-1,-1}};
        for (int[] d : diag) {
            int r = kr + d[0], c = kc + d[1];
            while (at(r, c) == '.') { r += d[0]; c += d[1]; }
            char p = at(r, c);
            if (p == 'B' || p == 'Q') return true;
        }
        int[][] kn = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
        for (int[] d : kn)
            if (at(kr + d[0], kc + d[1]) == 'N') return true;
        if (at(kr + 1, kc - 1) == 'P' || at(kr + 1, kc + 1) == 'P') return true;
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++)
                if ((dr != 0 || dc != 0) && at(kr + dr, kc + dc) == 'k') return true;
        return false;
    }

    public static void main(String[] args) {
        String[] board = {
            "...K....",
            "........",
            ".B......",
            "......P.",
            ".......R",
            "..N.....",
            "........",
            ".....Q.."
        };
        System.out.println(inCheck(board) ? "True" : "False");
    }
}
