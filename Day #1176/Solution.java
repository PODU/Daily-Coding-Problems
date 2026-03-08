// Day 1176: Word search in a 2D board via DFS backtracking.
// Try each cell as a start, explore 4 neighbors, mark visited in-place.
// Time O(M*N*4^L), Space O(L) recursion (L = word length).
public class Solution {
    static boolean dfs(char[][] b, String w, int k, int i, int j) {
        if (k == w.length()) return true;
        if (i < 0 || i >= b.length || j < 0 || j >= b[0].length || b[i][j] != w.charAt(k)) return false;
        char saved = b[i][j];
        b[i][j] = '#';
        boolean found = dfs(b, w, k+1, i+1, j) || dfs(b, w, k+1, i-1, j) ||
                        dfs(b, w, k+1, i, j+1) || dfs(b, w, k+1, i, j-1);
        b[i][j] = saved;
        return found;
    }

    static boolean exists(char[][] b, String w) {
        for (int i = 0; i < b.length; i++)
            for (int j = 0; j < b[0].length; j++)
                if (dfs(b, w, 0, i, j)) return true;
        return false;
    }

    public static void main(String[] args) {
        char[][] board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
        System.out.println(exists(board, "ABCCED") ? "true" : "false");
        System.out.println(exists(board, "SEE")    ? "true" : "false");
        System.out.println(exists(board, "ABCB")   ? "true" : "false");
    }
}
