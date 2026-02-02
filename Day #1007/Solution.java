// King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
// Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).
public class Solution {
    static boolean inBoard(int r, int c) { return r >= 0 && r < 8 && c >= 0 && c < 8; }

    static boolean isCheck(String[] b) {
        int kr = -1, kc = -1;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (b[r].charAt(c) == 'K') { kr = r; kc = c; }

        int[][] dirsRC = {{1,0},{-1,0},{0,1},{0,-1}};
        for (int[] d : dirsRC) {
            int r = kr + d[0], c = kc + d[1];
            while (inBoard(r, c)) {
                char p = b[r].charAt(c);
                if (p != '.') { if (p == 'R' || p == 'Q') return true; break; }
                r += d[0]; c += d[1];
            }
        }
        int[][] dirsD = {{1,1},{1,-1},{-1,1},{-1,-1}};
        for (int[] d : dirsD) {
            int r = kr + d[0], c = kc + d[1];
            while (inBoard(r, c)) {
                char p = b[r].charAt(c);
                if (p != '.') { if (p == 'B' || p == 'Q') return true; break; }
                r += d[0]; c += d[1];
            }
        }
        int[][] kn = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
        for (int[] d : kn) {
            int r = kr + d[0], c = kc + d[1];
            if (inBoard(r, c) && b[r].charAt(c) == 'N') return true;
        }
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++) {
                if (dr == 0 && dc == 0) continue;
                int r = kr + dr, c = kc + dc;
                if (inBoard(r, c) && b[r].charAt(c) == 'K') return true;
            }
        for (int dc = -1; dc <= 1; dc += 2) {
            int r = kr + 1, c = kc + dc;
            if (inBoard(r, c) && b[r].charAt(c) == 'P') return true;
        }
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
        System.out.println(isCheck(board) ? "True" : "False"); // True
    }
}
