// Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
// Time O(N*M*L), Space O(max(N,M)).
public class Solution {
    static boolean findWord(char[][] grid, String word) {
        int n = grid.length, m = grid[0].length;
        for (int r = 0; r < n; ++r) {
            StringBuilder row = new StringBuilder();
            for (int c = 0; c < m; ++c) row.append(grid[r][c]);
            if (row.indexOf(word) >= 0) return true;
        }
        for (int c = 0; c < m; ++c) {
            StringBuilder col = new StringBuilder();
            for (int r = 0; r < n; ++r) col.append(grid[r][c]);
            if (col.indexOf(word) >= 0) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        char[][] grid = {
            {'F','A','C','I'},
            {'O','B','Q','P'},
            {'A','N','O','B'},
            {'M','A','S','S'}
        };
        System.out.println("'FOAM' -> " + findWord(grid, "FOAM"));
        System.out.println("'MASS' -> " + findWord(grid, "MASS"));
    }
}
