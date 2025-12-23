// Word search L-to-R / U-to-D only: scan each row and column for target substring.
// Time O(R*C*L), Space O(max(R,C)).
public class Solution {
    static boolean findWord(char[][] m, String target) {
        int R = m.length, C = R == 0 ? 0 : m[0].length;
        for (int r = 0; r < R; r++) {
            StringBuilder row = new StringBuilder();
            for (int c = 0; c < C; c++) row.append(m[r][c]);
            if (row.toString().contains(target)) return true;
        }
        for (int c = 0; c < C; c++) {
            StringBuilder col = new StringBuilder();
            for (int r = 0; r < R; r++) col.append(m[r][c]);
            if (col.toString().contains(target)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        char[][] matrix = {
            {'F','A','C','I'},
            {'O','B','Q','P'},
            {'A','N','O','B'},
            {'M','A','S','S'}
        };
        System.out.println(findWord(matrix, "FOAM"));
    }
}
