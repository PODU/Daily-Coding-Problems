// Chess check detection: locate black king, test pawn/knight attacks and ray-cast
// bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.
public class Solution {
    static boolean inCheck(String[] b) {
        int kr = -1, kc = -1;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (b[r].charAt(c) == 'K') { kr = r; kc = c; }
        if (kr < 0) return false;

        // White pawns move up; a pawn at (kr+1, kc+-1) attacks the king.
        for (int dc = -1; dc <= 1; dc += 2) {
            int pr = kr + 1, pc = kc + dc;
            if (pr >= 0 && pr < 8 && pc >= 0 && pc < 8 && b[pr].charAt(pc) == 'P') return true;
        }

        int[][] km = {{1,2},{1,-2},{-1,2},{-1,-2},{2,1},{2,-1},{-2,1},{-2,-1}};
        for (int[] m : km) {
            int r = kr + m[0], c = kc + m[1];
            if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r].charAt(c) == 'N') return true;
        }

        int[][] diag = {{1,1},{1,-1},{-1,1},{-1,-1}};
        for (int[] d : diag) {
            int r = kr + d[0], c = kc + d[1];
            while (r >= 0 && r < 8 && c >= 0 && c < 8) {
                char p = b[r].charAt(c);
                if (p != '.') { if (p == 'B' || p == 'Q') return true; break; }
                r += d[0]; c += d[1];
            }
        }

        int[][] strt = {{1,0},{-1,0},{0,1},{0,-1}};
        for (int[] d : strt) {
            int r = kr + d[0], c = kc + d[1];
            while (r >= 0 && r < 8 && c >= 0 && c < 8) {
                char p = b[r].charAt(c);
                if (p != '.') { if (p == 'R' || p == 'Q') return true; break; }
                r += d[0]; c += d[1];
            }
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
        System.out.println(inCheck(board) ? "True" : "False");
    }
}
