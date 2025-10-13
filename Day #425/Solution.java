// Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
// with blocking. O(1) board scan from the king's square.
public class Solution {
    static boolean in(int r, int c) { return r >= 0 && r < 8 && c >= 0 && c < 8; }

    public static void main(String[] args) {
        String[] b = {
            "...K....",
            "........",
            ".B......",
            "......P.",
            ".......R",
            "..N.....",
            "........",
            ".....Q.."
        };
        int kr = -1, kc = -1;
        for (int i = 0; i < 8; i++)
            for (int j = 0; j < 8; j++)
                if (b[i].charAt(j) == 'K') { kr = i; kc = j; }

        boolean check = false;

        int[][] kn = {{-2,-1},{-2,1},{-1,-2},{-1,2},{1,-2},{1,2},{2,-1},{2,1}};
        for (int[] d : kn) { int r = kr + d[0], c = kc + d[1]; if (in(r, c) && b[r].charAt(c) == 'N') check = true; }

        for (int dc = -1; dc <= 1; dc += 2) { int r = kr + 1, c = kc + dc; if (in(r, c) && b[r].charAt(c) == 'P') check = true; }

        int[][] orth = {{-1,0},{1,0},{0,-1},{0,1}};
        for (int[] d : orth) {
            int r = kr + d[0], c = kc + d[1];
            while (in(r, c)) { char p = b[r].charAt(c); if (p != '.') { if (p == 'R' || p == 'Q') check = true; break; } r += d[0]; c += d[1]; }
        }

        int[][] diag = {{-1,-1},{-1,1},{1,-1},{1,1}};
        for (int[] d : diag) {
            int r = kr + d[0], c = kc + d[1];
            while (in(r, c)) { char p = b[r].charAt(c); if (p != '.') { if (p == 'B' || p == 'Q') check = true; break; } r += d[0]; c += d[1]; }
        }

        System.out.println(check ? "True" : "False");
    }
}
