// Day 1672: Determine if black king is in check on an 8x8 board.
// Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).
public class Solution {
    static boolean inCheck(String[] b) {
        int kr = -1, kc = -1;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (b[r].charAt(c) == 'K') { kr = r; kc = c; }

        int[][] km = {{-2,-1},{-2,1},{-1,-2},{-1,2},{1,-2},{1,2},{2,-1},{2,1}};
        for (int[] m : km) {
            int r = kr + m[0], c = kc + m[1];
            if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r].charAt(c) == 'N') return true;
        }
        int[][] dirs = {{1,0},{-1,0},{0,1},{0,-1},{1,1},{1,-1},{-1,1},{-1,-1}};
        for (int d = 0; d < 8; d++) {
            boolean diag = d >= 4;
            for (int step = 1; step < 8; step++) {
                int r = kr + dirs[d][0]*step, c = kc + dirs[d][1]*step;
                if (r < 0 || r >= 8 || c < 0 || c >= 8) break;
                char p = b[r].charAt(c);
                if (p == '.') continue;
                if (diag) {
                    if (p == 'B' || p == 'Q') return true;
                    if (step == 1 && p == 'K') return true;
                    if (step == 1 && p == 'P' && dirs[d][0] == 1) return true;
                } else {
                    if (p == 'R' || p == 'Q') return true;
                    if (step == 1 && p == 'K') return true;
                }
                break;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        String[] board = {
            "...K....","........",".B......","......P.",
            ".......R","..N.....","........",".....Q.."};
        System.out.println(inCheck(board) ? "True" : "False"); // True
    }
}
