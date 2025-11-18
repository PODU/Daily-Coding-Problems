// Word Search: DFS backtracking from each cell, mark visited then restore.
// Time O(M*N*4^L), Space O(L) recursion depth.
public class Solution {
    static boolean dfs(char[][] b, String w, int i, int j, int k) {
        if (k == w.length()) return true;
        if (i < 0 || j < 0 || i >= b.length || j >= b[0].length || b[i][j] != w.charAt(k)) return false;
        char tmp = b[i][j];
        b[i][j] = '#';
        boolean found = dfs(b, w, i+1, j, k+1) || dfs(b, w, i-1, j, k+1) ||
                        dfs(b, w, i, j+1, k+1) || dfs(b, w, i, j-1, k+1);
        b[i][j] = tmp;
        return found;
    }

    static boolean exists(char[][] board, String word) {
        for (int i = 0; i < board.length; i++)
            for (int j = 0; j < board[0].length; j++)
                if (dfs(board, word, i, j, 0)) return true;
        return false;
    }

    public static void main(String[] args) {
        char[][] board = {{'A','B','C','E'},{'S','F','C','S'},{'A','D','E','E'}};
        System.out.println(exists(board, "ABCCED"));
        System.out.println(exists(board, "SEE"));
        System.out.println(exists(board, "ABCB"));
    }
}
