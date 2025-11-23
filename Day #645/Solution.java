// Day 645: Find a word in a grid going left-to-right or top-to-bottom.
// Approach: scan every row and every column for the target as a substring start.
// Time: O(R*C*L), Space: O(1).
public class Solution {
    static boolean findWord(char[][] g, String word) {
        int R = g.length, C = g[0].length, L = word.length();
        for (int r = 0; r < R; r++)
            for (int c = 0; c + L <= C; c++) {
                boolean ok = true;
                for (int k = 0; k < L; k++) if (g[r][c+k] != word.charAt(k)) { ok = false; break; }
                if (ok) return true;
            }
        for (int c = 0; c < C; c++)
            for (int r = 0; r + L <= R; r++) {
                boolean ok = true;
                for (int k = 0; k < L; k++) if (g[r+k][c] != word.charAt(k)) { ok = false; break; }
                if (ok) return true;
            }
        return false;
    }

    public static void main(String[] args) {
        char[][] g = {
            {'F','A','C','I'},
            {'O','B','Q','P'},
            {'A','N','O','B'},
            {'M','A','S','S'}
        };
        System.out.println(findWord(g, "FOAM")); // true
        System.out.println(findWord(g, "MASS")); // true
    }
}
