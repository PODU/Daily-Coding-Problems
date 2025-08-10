// Day 98: Word search via DFS backtracking from each cell, marking visited cells
// in place. O(M*N*4^L) time, O(L) recursion space.
public class Solution {
    static boolean dfs(char[][] b, String w, int r, int c, int i) {
        if (i == w.length()) return true;
        if (r < 0 || r >= b.length || c < 0 || c >= b[0].length || b[r][c] != w.charAt(i))
            return false;
        char saved = b[r][c];
        b[r][c] = '#';
        boolean found = dfs(b, w, r + 1, c, i + 1) || dfs(b, w, r - 1, c, i + 1)
                     || dfs(b, w, r, c + 1, i + 1) || dfs(b, w, r, c - 1, i + 1);
        b[r][c] = saved;
        return found;
    }

    static boolean exists(char[][] b, String w) {
        for (int r = 0; r < b.length; r++)
            for (int c = 0; c < b[0].length; c++)
                if (dfs(b, w, r, c, 0)) return true;
        return false;
    }

    public static void main(String[] args) {
        char[][] board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
        System.out.println(exists(board, "ABCCED")); // true
        System.out.println(exists(board, "SEE"));    // true
        System.out.println(exists(board, "ABCB"));   // false
    }
}
